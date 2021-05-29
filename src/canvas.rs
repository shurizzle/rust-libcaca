use std::{ptr, slice, sync::Arc};

use libcaca_sys::{
    caca_blit, caca_canvas_t, caca_clear_canvas, caca_create_canvas, caca_disable_dirty_rect,
    caca_enable_dirty_rect, caca_flip, caca_flop, caca_free_canvas, caca_get_canvas_attrs,
    caca_get_canvas_chars, caca_get_canvas_handle_x, caca_get_canvas_handle_y,
    caca_get_canvas_height, caca_get_canvas_width, caca_get_char, caca_get_dirty_rect_count,
    caca_gotoxy, caca_invert, caca_put_char, caca_put_str, caca_set_canvas_boundaries,
    caca_set_canvas_handle, caca_set_canvas_size, caca_wherex, caca_wherey,
};

use crate::{result::Result, utils::lossy_cstring, Boundaries, Point, Rectangle};

pub(crate) struct InternalCanvas(*mut caca_canvas_t);

impl InternalCanvas {
    pub(crate) fn as_internal(&self) -> *mut caca_canvas_t {
        self.0
    }
}

impl Drop for InternalCanvas {
    fn drop(&mut self) {
        unsafe { caca_free_canvas(self.0) };
    }
}

pub struct Canvas(pub(crate) Arc<InternalCanvas>);

impl Canvas {
    pub fn new(boundaries: &Boundaries) -> Result<Canvas> {
        let ptr = unsafe { caca_create_canvas(boundaries.width as i32, boundaries.height as i32) };
        if ptr.is_null() {
            todo!("Error handling")
        } else {
            let internal = InternalCanvas(ptr);
            Ok(Self(Arc::new(internal)))
        }
    }

    pub fn set_size(&self, boundaries: &Boundaries) -> Result<()> {
        if unsafe {
            caca_set_canvas_size(
                self.as_internal(),
                boundaries.width as i32,
                boundaries.height as i32,
            )
        } != 0
        {
            todo!("Error handling")
        } else {
            Ok(())
        }
    }

    pub fn width(&self) -> usize {
        unsafe { caca_get_canvas_width(self.as_internal()) as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { caca_get_canvas_height(self.as_internal()) as usize }
    }

    pub fn size(&self) -> Boundaries {
        Boundaries {
            width: self.width(),
            height: self.height(),
        }
    }

    pub fn chars(&self) -> &[u32] {
        let len = self.width() * self.height();
        unsafe { slice::from_raw_parts(caca_get_canvas_chars(self.as_internal()), len) }
    }

    pub fn attrs(&self) -> &[u32] {
        let len = self.width() * self.height();
        unsafe { slice::from_raw_parts(caca_get_canvas_attrs(self.as_internal()), len) }
    }

    pub fn gotoxy(&self, point: &Point) {
        unsafe { caca_gotoxy(self.as_internal(), point.x as i32, point.y as i32) };
    }

    pub fn wherex(&self) -> usize {
        unsafe { caca_wherex(self.as_internal()) as usize }
    }

    pub fn wherey(&self) -> usize {
        unsafe { caca_wherey(self.as_internal()) as usize }
    }

    pub fn put_char(&self, point: &Point, ch: u32) -> usize {
        unsafe { caca_put_char(self.as_internal(), point.x as i32, point.y as i32, ch) as usize }
    }

    pub fn get_char(&self, point: &Point) -> u32 {
        unsafe { caca_get_char(self.as_internal(), point.x as i32, point.y as i32) }
    }

    pub fn put_str<S: AsRef<str>>(&self, point: &Point, s: S) -> usize {
        let c_str = lossy_cstring(s);
        unsafe {
            caca_put_str(
                self.as_internal(),
                point.x as i32,
                point.y as i32,
                c_str.as_ptr(),
            ) as usize
        }
    }

    // Skipping useless functions:
    // - caca_printf
    // - caca_vprintf

    pub fn clear(&self) {
        unsafe { caca_clear_canvas(self.as_internal()) };
    }

    pub fn set_handle(&self, point: Point) {
        unsafe { caca_set_canvas_handle(self.as_internal(), point.x as i32, point.y as i32) };
    }

    pub fn get_handle_x(&self) -> usize {
        unsafe { caca_get_canvas_handle_x(self.as_internal()) as usize }
    }

    pub fn get_handle_y(&self) -> usize {
        unsafe { caca_get_canvas_handle_y(self.as_internal()) as usize }
    }

    pub fn get_hande(&self) -> Point {
        Point {
            x: self.get_handle_x(),
            y: self.get_handle_y(),
        }
    }

    pub fn blit(&self, point: &Point, src: &Canvas, mask: Option<&Canvas>) -> Result<()> {
        if unsafe {
            caca_blit(
                self.as_internal(),
                point.x as i32,
                point.y as i32,
                src.as_internal(),
                mask.map(Canvas::as_internal).unwrap_or(ptr::null_mut()),
            )
        } != 0
        {
            todo!()
        }

        Ok(())
    }

    pub fn set_boundaries(&self, boundaries: &Rectangle) -> Result<()> {
        if unsafe {
            caca_set_canvas_boundaries(
                self.as_internal(),
                boundaries.x as i32,
                boundaries.y as i32,
                boundaries.width as i32,
                boundaries.height as i32,
            )
        } != 0
        {
            todo!()
        }

        Ok(())
    }

    pub fn disable_dirty_rect(&self) {
        unsafe { caca_disable_dirty_rect(self.as_internal()) };
    }

    pub fn enable_dirty_rect(&self) -> Result<()> {
        if unsafe { caca_enable_dirty_rect(self.as_internal()) } != 0 {
            todo!()
        }

        Ok(())
    }

    pub fn get_dirty_rect_count(&self) -> usize {
        unsafe { caca_get_dirty_rect_count(self.as_internal()) as usize }
    }

    // TODO: Annoying dirty rects things

    pub fn invert(&self) {
        unsafe { caca_invert(self.as_internal()) };
    }

    pub fn flip(&self) {
        unsafe { caca_flip(self.as_internal()) };
    }

    pub fn flop(&self) {
        unsafe { caca_flop(self.as_internal()) };
    }

    pub(crate) fn as_internal(&self) -> *mut caca_canvas_t {
        self.0.as_internal()
    }
}

impl Clone for Canvas {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
