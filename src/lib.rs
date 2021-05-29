use std::{borrow::Cow, ffi::CStr, str};

use libcaca_sys::{caca_get_version, caca_rand};

mod canvas;
pub mod error;
pub mod result;
mod utils;

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

pub use canvas::Canvas;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Boundaries {
    pub width: usize,
    pub height: usize,
}

pub type Bounds = Boundaries;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rectangle {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub type Rect = Rectangle;
