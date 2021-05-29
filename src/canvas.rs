use std::{ffi::CStr, path::Path, ptr, slice, str, sync::Arc};

use errno::errno;
use libcaca_sys::{
    caca_add_dirty_rect, caca_blit, caca_canvas_set_figfont, caca_canvas_t, caca_clear_canvas,
    caca_clear_dirty_rect_list, caca_create_canvas, caca_create_frame, caca_disable_dirty_rect,
    caca_draw_box, caca_draw_circle, caca_draw_ellipse, caca_draw_line, caca_draw_polyline,
    caca_draw_thin_box, caca_draw_thin_ellipse, caca_draw_thin_line, caca_draw_thin_polyline,
    caca_draw_thin_triangle, caca_draw_triangle, caca_enable_dirty_rect, caca_fill_ellipse,
    caca_fill_triangle, caca_flip, caca_flop, caca_flush_figlet, caca_free_canvas, caca_free_frame,
    caca_get_attr, caca_get_canvas_attrs, caca_get_canvas_chars, caca_get_canvas_handle_x,
    caca_get_canvas_handle_y, caca_get_canvas_height, caca_get_canvas_width, caca_get_char,
    caca_get_dirty_rect, caca_get_dirty_rect_count, caca_get_frame_count, caca_get_frame_name,
    caca_gotoxy, caca_invert, caca_put_attr, caca_put_char, caca_put_figchar, caca_put_str,
    caca_remove_dirty_rect, caca_rotate_180, caca_rotate_left, caca_rotate_right, caca_set_attr,
    caca_set_canvas_boundaries, caca_set_canvas_handle, caca_set_canvas_size, caca_set_color_ansi,
    caca_set_color_argb, caca_set_frame, caca_set_frame_name, caca_stretch_left,
    caca_stretch_right, caca_toggle_attr, caca_unset_attr, caca_wherex, caca_wherey,
};

use crate::{
    attr::Attr, error::Error, result::Result, utils::lossy_cstring, Boundaries, Circle, Ellipse,
    Point, Rectangle, Triangle,
};

pub(crate) enum InternalCanvas {
    Borrowed(*mut caca_canvas_t),
    Owned(*mut caca_canvas_t),
}

impl InternalCanvas {
    pub(crate) fn as_internal(&self) -> *mut caca_canvas_t {
        match self {
            InternalCanvas::Borrowed(ptr) => *ptr,
            InternalCanvas::Owned(ptr) => *ptr,
        }
    }
}

impl Drop for InternalCanvas {
    fn drop(&mut self) {
        match self {
            InternalCanvas::Owned(ptr) => {
                unsafe { caca_free_canvas(*ptr) };
            }
            _ => (),
        }
    }
}

pub struct Canvas(pub(crate) Arc<InternalCanvas>);

impl Canvas {
    pub fn new(boundaries: &Boundaries) -> Result<Canvas> {
        let ptr = unsafe { caca_create_canvas(boundaries.width as i32, boundaries.height as i32) };
        if ptr.is_null() {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidSize),
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            let internal = InternalCanvas::Owned(ptr);
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
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidSize),
                libc::EBUSY => Err(Error::CanvasInUse),
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
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

    pub fn attrs(&self) -> &[Attr] {
        let len = self.width() * self.height();
        unsafe {
            slice::from_raw_parts(
                caca_get_canvas_attrs(self.as_internal()) as *const Attr,
                len,
            )
        }
    }

    pub fn gotoxy(&self, point: &Point) {
        unsafe { caca_gotoxy(self.as_internal(), point.x as i32, point.y as i32) };
    }

    pub fn wherex(&self) -> i32 {
        unsafe { caca_wherex(self.as_internal()) }
    }

    pub fn wherey(&self) -> i32 {
        unsafe { caca_wherey(self.as_internal()) }
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

    pub fn get_handle_x(&self) -> i32 {
        unsafe { caca_get_canvas_handle_x(self.as_internal()) }
    }

    pub fn get_handle_y(&self) -> i32 {
        unsafe { caca_get_canvas_handle_y(self.as_internal()) }
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
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidMaskSize),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
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
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidSize),
                libc::EBUSY => Err(Error::CanvasInUse),
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn disable_dirty_rect(&self) {
        unsafe { caca_disable_dirty_rect(self.as_internal()) };
    }

    pub fn enable_dirty_rect(&self) -> Result<()> {
        if unsafe { caca_enable_dirty_rect(self.as_internal()) } != 0 {
            match errno().0 {
                // Just ignore this error and silently do nothing if dirty rects
                // are already enabled
                libc::EINVAL => Ok(()),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn get_dirty_rect_count(&self) -> usize {
        unsafe { caca_get_dirty_rect_count(self.as_internal()) as usize }
    }

    pub fn get_dirty_rect(&self, index: i32) -> Result<Rectangle> {
        let mut x = 0i32;
        let mut y = 0i32;
        let mut width = 0i32;
        let mut height = 0i32;

        if unsafe {
            caca_get_dirty_rect(
                self.as_internal(),
                index,
                &mut x as *mut _,
                &mut y as *mut _,
                &mut width as *mut _,
                &mut height as *mut _,
            )
        } != 0
        {
            match errno().0 {
                libc::EINVAL => Err(Error::OutOfBounds),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(Rectangle {
                x,
                y,
                width: width as usize,
                height: height as usize,
            })
        }
    }

    pub fn caca_add_dirty_rect(&self, rectangle: &Rectangle) -> Result<()> {
        if unsafe {
            caca_add_dirty_rect(
                self.as_internal(),
                rectangle.x as i32,
                rectangle.y as i32,
                rectangle.width as i32,
                rectangle.height as i32,
            )
        } != 0
        {
            match errno().0 {
                libc::EINVAL => Err(Error::OutOfBounds),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn remove_dirty_rect(&self, rectangle: &Rectangle) -> Result<()> {
        if unsafe {
            caca_remove_dirty_rect(
                self.as_internal(),
                rectangle.x as i32,
                rectangle.y as i32,
                rectangle.width as i32,
                rectangle.height as i32,
            )
        } != 0
        {
            match errno().0 {
                libc::EINVAL => Err(Error::OutOfBounds),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn clear_dirty_rect_list(&self) {
        unsafe { caca_clear_dirty_rect_list(self.as_internal()) };
    }

    pub fn invert(&self) {
        unsafe { caca_invert(self.as_internal()) };
    }

    pub fn flip(&self) {
        unsafe { caca_flip(self.as_internal()) };
    }

    pub fn flop(&self) {
        unsafe { caca_flop(self.as_internal()) };
    }

    pub fn rotate_180(&self) {
        unsafe { caca_rotate_180(self.as_internal()) };
    }

    pub fn rotate_left(&self) -> Result<()> {
        if unsafe { caca_rotate_left(self.as_internal()) } != 0 {
            match errno().0 {
                libc::EBUSY => Err(Error::CanvasInUse),
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn rotate_right(&self) -> Result<()> {
        if unsafe { caca_rotate_right(self.as_internal()) } != 0 {
            match errno().0 {
                libc::EBUSY => Err(Error::CanvasInUse),
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn stretch_left(&self) -> Result<()> {
        if unsafe { caca_stretch_left(self.as_internal()) } != 0 {
            match errno().0 {
                libc::EBUSY => Err(Error::CanvasInUse),
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn stretch_right(&self) -> Result<()> {
        if unsafe { caca_stretch_right(self.as_internal()) } != 0 {
            match errno().0 {
                libc::EBUSY => Err(Error::CanvasInUse),
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn get_attr(&self, point: &Point) -> Attr {
        unsafe { caca_get_attr(self.as_internal(), point.x as i32, point.y as i32) }.into()
    }

    pub fn set_attr<A: Into<Attr>>(&self, attr: A) {
        unsafe { caca_set_attr(self.as_internal(), attr.into().into()) };
    }

    pub fn unset_attr<A: Into<Attr>>(&self, attr: A) {
        unsafe { caca_unset_attr(self.as_internal(), attr.into().into()) };
    }

    pub fn toggle_attr<A: Into<Attr>>(&self, attr: A) {
        unsafe { caca_toggle_attr(self.as_internal(), attr.into().into()) };
    }

    pub fn put_attr<A: Into<Attr>>(&self, point: &Point, attr: A) {
        unsafe {
            caca_put_attr(
                self.as_internal(),
                point.x as i32,
                point.y as i32,
                attr.into().into(),
            )
        };
    }

    pub fn set_color_ansi(&self, fg: u8, bg: u8) -> Result<()> {
        if unsafe { caca_set_color_ansi(self.as_internal(), fg, bg) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidColor),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn set_color_argb(&self, fg: u16, bg: u16) {
        unsafe { caca_set_color_argb(self.as_internal(), fg, bg) };
    }

    pub fn draw_line(&self, point1: &Point, point2: &Point, ch: u32) {
        unsafe {
            caca_draw_line(
                self.as_internal(),
                point1.x,
                point1.y,
                point2.x,
                point2.y,
                ch,
            )
        };
    }

    pub fn draw_polyline(&self, points: &[Point], ch: u32) {
        let mut xs = Vec::with_capacity(points.len());
        let mut ys = Vec::with_capacity(points.len());

        for p in points {
            xs.push(p.x);
            ys.push(p.y);
        }

        unsafe {
            caca_draw_polyline(
                self.as_internal(),
                xs.as_ptr(),
                ys.as_ptr(),
                points.len() as i32,
                ch,
            )
        };
    }

    pub fn draw_thin_line(&self, point1: &Point, point2: &Point) {
        unsafe { caca_draw_thin_line(self.as_internal(), point1.x, point1.y, point2.x, point2.y) };
    }

    pub fn draw_thin_polyline(&self, points: &[Point]) {
        let mut xs = Vec::with_capacity(points.len());
        let mut ys = Vec::with_capacity(points.len());

        for p in points {
            xs.push(p.x);
            ys.push(p.y);
        }

        unsafe {
            caca_draw_thin_polyline(
                self.as_internal(),
                xs.as_ptr(),
                ys.as_ptr(),
                points.len() as i32,
            )
        };
    }

    pub fn draw_circle(&self, circle: &Circle, ch: u32) {
        unsafe {
            caca_draw_circle(
                self.as_internal(),
                circle.x,
                circle.y,
                circle.radius as i32,
                ch,
            )
        };
    }

    pub fn draw_ellipse(&self, ellipse: &Ellipse, ch: u32) {
        unsafe {
            caca_draw_ellipse(
                self.as_internal(),
                ellipse.x,
                ellipse.y,
                ellipse.x_radius as i32,
                ellipse.y_radius as i32,
                ch,
            )
        };
    }

    pub fn draw_thin_ellipse(&self, ellipse: &Ellipse) {
        unsafe {
            caca_draw_thin_ellipse(
                self.as_internal(),
                ellipse.x,
                ellipse.y,
                ellipse.x_radius as i32,
                ellipse.y_radius as i32,
            )
        };
    }

    pub fn fill_ellipse(&self, ellipse: &Ellipse, ch: u32) {
        unsafe {
            caca_fill_ellipse(
                self.as_internal(),
                ellipse.x,
                ellipse.y,
                ellipse.x_radius as i32,
                ellipse.y_radius as i32,
                ch,
            )
        };
    }

    pub fn draw_box(&self, rect: &Rectangle, ch: u32) {
        unsafe {
            caca_draw_box(
                self.as_internal(),
                rect.x,
                rect.y,
                rect.width as i32,
                rect.height as i32,
                ch,
            )
        };
    }

    pub fn draw_thin_box(&self, rect: &Rectangle) {
        unsafe {
            caca_draw_thin_box(
                self.as_internal(),
                rect.x,
                rect.y,
                rect.width as i32,
                rect.height as i32,
            )
        };
    }

    pub fn draw_triangle(&self, triangle: &Triangle, ch: u32) {
        unsafe {
            caca_draw_triangle(
                self.as_internal(),
                triangle.vertex1.x,
                triangle.vertex1.y,
                triangle.vertex2.x,
                triangle.vertex2.y,
                triangle.vertex3.x,
                triangle.vertex3.y,
                ch,
            )
        };
    }

    pub fn draw_thin_triangle(&self, triangle: &Triangle) {
        unsafe {
            caca_draw_thin_triangle(
                self.as_internal(),
                triangle.vertex1.x,
                triangle.vertex1.y,
                triangle.vertex2.x,
                triangle.vertex2.y,
                triangle.vertex3.x,
                triangle.vertex3.y,
            )
        };
    }

    pub fn fill_triangle(&self, triangle: &Triangle, ch: u32) {
        unsafe {
            caca_fill_triangle(
                self.as_internal(),
                triangle.vertex1.x,
                triangle.vertex1.y,
                triangle.vertex2.x,
                triangle.vertex2.y,
                triangle.vertex3.x,
                triangle.vertex3.y,
                ch,
            )
        };
    }

    // TODO: fill_triangle_textured, idk what's uv and why it's a float array

    pub fn frame_count(&self) -> usize {
        unsafe { caca_get_frame_count(self.as_internal()) as usize }
    }

    pub fn frame_name(&self) -> String {
        unsafe { CStr::from_ptr(caca_get_frame_name(self.as_internal())) }
            .to_string_lossy()
            .to_string()
    }

    pub fn set_frame(&self, id: usize) -> Result<()> {
        if unsafe { caca_set_frame(self.as_internal(), id as i32) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidFrameIndex),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn set_frame_name<S: AsRef<str>>(&self, name: S) -> Result<()> {
        if unsafe { caca_set_frame_name(self.as_internal(), lossy_cstring(name).as_ptr()) } != 0 {
            match errno().0 {
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn create_frame(&self, id: usize) -> Result<()> {
        if unsafe { caca_create_frame(self.as_internal(), id as i32) } != 0 {
            match errno().0 {
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn free_frame(&self, id: usize) -> Result<()> {
        if unsafe { caca_free_frame(self.as_internal(), id as i32) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidFrameIndex),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn set_figfont<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if unsafe {
            caca_canvas_set_figfont(
                self.as_internal(),
                path.as_ref().to_string_lossy().as_ptr() as *const _,
            )
        } != 0
        {
            Err(Error::InvalidFIGfont)
        } else {
            Ok(())
        }
    }

    pub fn put_figchar(&self, ch: u32) -> Result<()> {
        if unsafe { caca_put_figchar(self.as_internal(), ch) } != 0 {
            Err(Error::InvalidFIGfont)
        } else {
            Ok(())
        }
    }

    pub fn flush_figlet(&self) -> Result<()> {
        if unsafe { caca_flush_figlet(self.as_internal()) } != 0 {
            Err(Error::InvalidFIGfont)
        } else {
            Ok(())
        }
    }

    // TODO: import/export

    pub(crate) fn as_internal(&self) -> *mut caca_canvas_t {
        self.0.as_internal()
    }
}

impl Clone for Canvas {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
