use std::{borrow::Cow, ffi::CStr, str};

use libcaca_sys::{caca_get_version, caca_rand};

mod attr;
mod canvas;
mod display;
pub mod error;
mod file;
pub mod result;
mod utils;

pub use attr::Argb;
pub use attr::Attr;
pub use canvas::Canvas;
pub use display::Display;
pub use file::File;

pub mod prelude {
    pub use crate::error::Error;
    pub use crate::result::Result;
}

pub fn rand(min: i32, max: i32) -> i32 {
    unsafe { caca_rand(min, max) }
}

pub fn version() -> Cow<'static, str> {
    unsafe { CStr::from_ptr(caca_get_version()).to_string_lossy() }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Boundaries {
    pub width: usize,
    pub height: usize,
}

pub type Bounds = Boundaries;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize,
}

pub type Rect = Rectangle;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ellipse {
    pub x: i32,
    pub y: i32,
    pub x_radius: usize,
    pub y_radius: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Triangle {
    pub vertex1: Point,
    pub vertex2: Point,
    pub vertex3: Point,
}
