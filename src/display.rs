use errno::errno;
use libcaca_sys::{
    caca_canvas_t, caca_create_display, caca_create_display_with_driver, caca_display_t,
    caca_event, caca_free_display, caca_get_canvas, caca_get_display_driver,
    caca_get_display_driver_list, caca_get_display_height, caca_get_display_time,
    caca_get_display_width, caca_get_event, caca_refresh_display, caca_set_cursor,
    caca_set_display_time, caca_set_display_title, caca_set_mouse,
};

use crate::{
    canvas::Canvas, error::Error, event::Event, event::EventMask, result::Result,
    utils::lossy_cstring, Boundaries,
};
use std::{borrow::Cow, ffi::CStr, marker::PhantomData, mem, ptr, time::Duration};

pub struct Display<'a>(*mut caca_display_t, Option<Canvas<'a>>);

impl<'a> Display<'a> {
    pub fn new(canvas: Option<Canvas>) -> Result<Display> {
        let (c_ptr, canvas) = Self::ptr_and_canvas(canvas)?;
        let internal = unsafe { caca_create_display(c_ptr) };

        Ok(Display(internal, canvas))
    }

    pub fn new_with_driver<S: AsRef<str>>(
        canvas: Option<Canvas>,
        driver_name: S,
    ) -> Result<Display> {
        let driver_name = lossy_cstring(driver_name);
        let (c_ptr, canvas) = Self::ptr_and_canvas(canvas)?;
        let internal = unsafe { caca_create_display_with_driver(c_ptr, driver_name.as_ptr()) };

        Ok(Display(internal, canvas))
    }

    fn ptr_and_canvas(canvas: Option<Canvas>) -> Result<(*mut caca_canvas_t, Option<Canvas>)> {
        match canvas {
            Some(canvas) => match canvas {
                Canvas::Borrowed(_, _) => Err(Error::CanvasInUse),
                Canvas::Owned(_) => Ok((canvas.as_internal(), Some(canvas))),
            },
            None => Ok((ptr::null_mut(), None)),
        }
    }

    pub fn driver(&self) -> Cow<'static, str> {
        let driver = unsafe { caca_get_display_driver(self.as_internal()) };
        unsafe { CStr::from_ptr(driver) }.to_string_lossy()
    }

    pub fn drivers() -> Vec<Cow<'static, str>> {
        let mut list = unsafe { caca_get_display_driver_list() };
        let mut res = Vec::new();

        while !(unsafe { *list }).is_null() {
            let driver = unsafe { *list };
            res.push(unsafe { CStr::from_ptr(driver) }.to_string_lossy());

            list = unsafe { list.add(mem::size_of::<*const i8>()) };
        }

        res.into_iter().filter(|x| !x.is_empty()).collect()
    }

    pub fn canvas(&'a self) -> Canvas<'a> {
        let ptr = unsafe { caca_get_canvas(self.as_internal()) };
        Canvas::Borrowed(ptr, PhantomData)
    }

    pub fn refresh(&self) {
        unsafe { caca_refresh_display(self.as_internal()) };
    }

    pub fn set_time(&self, time: Duration) -> Result<()> {
        let time = time.as_micros() as i32;

        if unsafe { caca_set_display_time(self.as_internal(), time) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidRefreshDelay),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn time(&self) -> Duration {
        Duration::from_micros(unsafe { caca_get_display_time(self.as_internal()) as u64 })
    }

    pub fn width(&self) -> usize {
        unsafe { caca_get_display_width(self.as_internal()) as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { caca_get_display_height(self.as_internal()) as usize }
    }

    pub fn size(&self) -> Boundaries {
        Boundaries {
            width: self.width(),
            height: self.height(),
        }
    }

    pub fn set_title<S: AsRef<str>>(&self, title: S) -> Result<()> {
        let title = lossy_cstring(title);

        if unsafe { caca_set_display_title(self.as_internal(), title.as_ptr()) } != 0 {
            match errno().0 {
                libc::ENOSYS => Err(Error::WindowTitleUnsupported),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn set_mouse(&self, flag: i32) -> Result<()> {
        if unsafe { caca_set_mouse(self.as_internal(), flag) } != 0 {
            match errno().0 {
                libc::ENOSYS => Err(Error::MousePointerUnsupported),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn set_cursor(&self, flag: i32) -> Result<()> {
        if unsafe { caca_set_cursor(self.as_internal(), flag) } != 0 {
            match errno().0 {
                libc::ENOSYS => Err(Error::MouseCursorUnsupported),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn next_event(&self, mask: EventMask) -> Event {
        let mut raw_event: caca_event = unsafe { mem::zeroed() };
        unsafe {
            caca_get_event(
                self.as_internal(),
                mask.bits(),
                &mut raw_event as *mut _,
                -1,
            )
        };
        raw_event.into()
    }

    pub fn poll_event(&self, mask: EventMask, timeout: Duration) -> Option<Event> {
        let timeout = timeout.as_micros();
        let timeout = if timeout > (u16::MAX as u128) {
            u16::MAX
        } else {
            timeout as u16
        };

        let mut raw_event: caca_event = unsafe { mem::zeroed() };

        if unsafe {
            caca_get_event(
                self.as_internal(),
                mask.bits(),
                &mut raw_event as *mut _,
                timeout as i32,
            )
        } == 0
        {
            None
        } else {
            Some(raw_event.into())
        }
    }

    pub(crate) fn as_internal(&self) -> *mut caca_display_t {
        self.0
    }
}

unsafe impl<'a> Send for Display<'a> {}

impl<'a> Drop for Display<'a> {
    fn drop(&mut self) {
        unsafe { caca_free_display(self.as_internal()) };
    }
}

#[cfg(test)]
mod tests {
    use super::Display;

    #[test]
    fn drivers() {
        println!("{:#?}", Display::drivers());
        println!(
            "{:#?}",
            Display::new_with_driver(None, "raw").unwrap().driver()
        );
    }
}
