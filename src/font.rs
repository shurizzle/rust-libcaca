use std::{borrow::Cow, ffi::CStr, mem, slice};

use errno::errno;
use libcaca_sys::{
    caca_font_t, caca_free_font, caca_get_font_blocks, caca_get_font_height, caca_get_font_list,
    caca_get_font_width, caca_load_font,
};

use crate::{error::Error, result::Result, utils::lossy_cstring, Boundaries};

pub struct Font(*mut caca_font_t);

impl Font {
    pub fn list() -> Vec<Cow<'static, str>> {
        let mut list = unsafe { caca_get_font_list() };
        let mut res = Vec::new();

        while !(unsafe { *list }).is_null() {
            let font = unsafe { *list };
            res.push(unsafe { CStr::from_ptr(font) }.to_string_lossy());

            list = unsafe { list.add(mem::size_of::<*const i8>()) };
        }

        res.into_iter().filter(|x| !x.is_empty()).collect()
    }

    pub fn new<S: AsRef<str>>(name: S) -> Result<Self> {
        let name = lossy_cstring(name);

        let internal = unsafe { caca_load_font(name.as_ptr() as *const _, 0u64) };

        if internal.is_null() {
            match errno().0 {
                libc::ENOENT => Err(Error::BuiltinFontNotFound),
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(Self(internal))
        }
    }

    pub fn load<T: Into<Vec<u8>>>(buffer: T) -> Result<Self> {
        let buffer: Vec<u8> = buffer.into();

        let internal = unsafe { caca_load_font(buffer.as_ptr() as *const _, buffer.len() as u64) };

        if internal.is_null() {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidFont),
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(Self(internal))
        }
    }

    pub fn width(&self) -> usize {
        unsafe { caca_get_font_width(self.as_internal()) as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { caca_get_font_height(self.as_internal()) as usize }
    }

    pub fn size(&self) -> Boundaries {
        Boundaries {
            width: self.width(),
            height: self.height(),
        }
    }

    pub fn blocks(&self) -> Vec<u32> {
        let base = unsafe { caca_get_font_blocks(self.as_internal()) };
        let mut last = base;

        while unsafe { *last } != 0 {
            last = unsafe { last.add(mem::size_of::<u32>()) };
        }

        let len = ((last as usize) - (base as usize)) / mem::size_of::<u32>();

        let s = unsafe { slice::from_raw_parts(base, len) };

        let mut res = Vec::with_capacity(s.len());
        res.extend_from_slice(s);
        res
    }

    pub(crate) fn as_internal(&self) -> *mut caca_font_t {
        self.0
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { caca_free_font(self.as_internal()) };
    }
}

#[cfg(test)]
mod tests {
    use super::Font;
    use std::borrow::Cow;

    #[test]
    fn test() {
        let _list: Vec<Cow<'static, str>> = Font::list();
        let font = Font::new(Font::list().first().unwrap()).unwrap();
        let _blocks = font.blocks();
    }
}
