use std::convert::TryFrom;
use std::{borrow::Cow, ffi::CStr, str};

use bitflags::bitflags;
use libcaca_sys::{
    caca_color, caca_color_CACA_BLACK, caca_color_CACA_BLUE, caca_color_CACA_BROWN,
    caca_color_CACA_CYAN, caca_color_CACA_DARKGRAY, caca_color_CACA_DEFAULT, caca_color_CACA_GREEN,
    caca_color_CACA_LIGHTBLUE, caca_color_CACA_LIGHTCYAN, caca_color_CACA_LIGHTGRAY,
    caca_color_CACA_LIGHTGREEN, caca_color_CACA_LIGHTMAGENTA, caca_color_CACA_LIGHTRED,
    caca_color_CACA_MAGENTA, caca_color_CACA_RED, caca_color_CACA_TRANSPARENT,
    caca_color_CACA_WHITE, caca_color_CACA_YELLOW, caca_style, caca_style_CACA_BLINK,
    caca_style_CACA_BOLD, caca_style_CACA_ITALICS, caca_style_CACA_UNDERLINE,
};
use libcaca_sys::{caca_get_version, caca_rand};

mod attr;
mod canvas;
mod display;
pub mod error;
pub mod event;
mod file;
pub mod result;
mod utils;

pub use attr::Argb;
pub use attr::Attr;
pub use canvas::Canvas;
pub use display::Display;
pub use event::Event;
pub use event::EventMask;
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

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black = caca_color_CACA_BLACK,
    Blue = caca_color_CACA_BLUE,
    Green = caca_color_CACA_GREEN,
    Cyan = caca_color_CACA_CYAN,
    Red = caca_color_CACA_RED,
    Magenta = caca_color_CACA_MAGENTA,
    Brown = caca_color_CACA_BROWN,
    LightGray = caca_color_CACA_LIGHTGRAY,
    DarkGray = caca_color_CACA_DARKGRAY,
    LightBlue = caca_color_CACA_LIGHTBLUE,
    LightGreen = caca_color_CACA_LIGHTGREEN,
    LightCyan = caca_color_CACA_LIGHTCYAN,
    LightRed = caca_color_CACA_LIGHTRED,
    LightMagenta = caca_color_CACA_LIGHTMAGENTA,
    Yellow = caca_color_CACA_YELLOW,
    White = caca_color_CACA_WHITE,
    Default = caca_color_CACA_DEFAULT,
    Transparent = caca_color_CACA_TRANSPARENT,
}

impl TryFrom<caca_color> for Color {
    type Error = crate::error::Error;

    #[allow(non_upper_case_globals)]
    fn try_from(c: caca_color) -> std::result::Result<Self, <Self as TryFrom<caca_color>>::Error> {
        match c {
            caca_color_CACA_BLACK => Ok(Color::Black),
            caca_color_CACA_BLUE => Ok(Color::Blue),
            caca_color_CACA_GREEN => Ok(Color::Green),
            caca_color_CACA_CYAN => Ok(Color::Cyan),
            caca_color_CACA_RED => Ok(Color::Red),
            caca_color_CACA_MAGENTA => Ok(Color::Magenta),
            caca_color_CACA_BROWN => Ok(Color::Brown),
            caca_color_CACA_LIGHTGRAY => Ok(Color::LightGray),
            caca_color_CACA_DARKGRAY => Ok(Color::DarkGray),
            caca_color_CACA_LIGHTBLUE => Ok(Color::LightBlue),
            caca_color_CACA_LIGHTGREEN => Ok(Color::LightGreen),
            caca_color_CACA_LIGHTCYAN => Ok(Color::LightCyan),
            caca_color_CACA_LIGHTRED => Ok(Color::LightRed),
            caca_color_CACA_LIGHTMAGENTA => Ok(Color::LightMagenta),
            caca_color_CACA_YELLOW => Ok(Color::Yellow),
            caca_color_CACA_WHITE => Ok(Color::White),
            caca_color_CACA_DEFAULT => Ok(Color::Default),
            caca_color_CACA_TRANSPARENT => Ok(Color::Transparent),
            _ => Err(crate::error::Error::InvalidColor),
        }
    }
}

impl Into<caca_color> for Color {
    fn into(self) -> caca_color {
        self as caca_color
    }
}

bitflags! {
    #[repr(C)]
    pub struct Style: caca_style {
        const BOLD = caca_style_CACA_BOLD;
        const ITALICS = caca_style_CACA_ITALICS;
        const UNDERLINE = caca_style_CACA_UNDERLINE;
        const BLINK = caca_style_CACA_BLINK;
    }
}
