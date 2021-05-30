use std::ffi::CStr;

use bitflags::bitflags;
use libcaca_sys::{
    caca_event, caca_event_type, caca_event_type_CACA_EVENT_ANY,
    caca_event_type_CACA_EVENT_KEY_PRESS, caca_event_type_CACA_EVENT_KEY_RELEASE,
    caca_event_type_CACA_EVENT_MOUSE_MOTION, caca_event_type_CACA_EVENT_MOUSE_PRESS,
    caca_event_type_CACA_EVENT_MOUSE_RELEASE, caca_event_type_CACA_EVENT_QUIT,
    caca_event_type_CACA_EVENT_RESIZE, caca_get_event_key_ch, caca_get_event_key_utf32,
    caca_get_event_key_utf8, caca_get_event_mouse_button, caca_get_event_mouse_x,
    caca_get_event_mouse_y, caca_get_event_resize_height, caca_get_event_resize_width,
    caca_get_event_type,
};

use crate::{Boundaries, Point};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    None,
    KeyPress(Key, u32, Option<String>),
    KeyRelease(Key, u32, Option<String>),
    MousePress(MouseButton),
    MouseRelease(MouseButton),
    MouseMotion(Point),
    Resize(Boundaries),
    Quit,
}

bitflags! {
    pub struct EventMask: i32 {
        const KEY_PRESS = caca_event_type_CACA_EVENT_KEY_PRESS as i32;
        const KEY_RELEASE = caca_event_type_CACA_EVENT_KEY_RELEASE as i32;
        const MOUSE_PRESS = caca_event_type_CACA_EVENT_MOUSE_PRESS as i32;
        const MOUSE_RELEASE = caca_event_type_CACA_EVENT_MOUSE_RELEASE as i32;
        const MOUSE_MOTION = caca_event_type_CACA_EVENT_MOUSE_MOTION as i32;
        const RESIZE = caca_event_type_CACA_EVENT_RESIZE as i32;
        const QUIT = caca_event_type_CACA_EVENT_QUIT as i32;

        const ANY = caca_event_type_CACA_EVENT_ANY as i32;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Key {
    Backspace,
    Tab,
    Return,
    Pause,
    Escape,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,

    Char(char),
    Ctrl(char),
    F(u32),
    Unknown(i32),
}

impl From<i32> for Key {
    fn from(raw_code: i32) -> Self {
        match raw_code as u32 {
            libcaca_sys::caca_key_CACA_KEY_CTRL_A => Key::Ctrl('a'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_B => Key::Ctrl('b'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_C => Key::Ctrl('c'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_D => Key::Ctrl('d'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_E => Key::Ctrl('e'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_F => Key::Ctrl('f'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_G => Key::Ctrl('g'),
            libcaca_sys::caca_key_CACA_KEY_BACKSPACE => Key::Backspace,
            libcaca_sys::caca_key_CACA_KEY_TAB => Key::Tab,
            libcaca_sys::caca_key_CACA_KEY_CTRL_J => Key::Ctrl('j'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_K => Key::Ctrl('k'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_L => Key::Ctrl('l'),
            libcaca_sys::caca_key_CACA_KEY_RETURN => Key::Return,
            libcaca_sys::caca_key_CACA_KEY_CTRL_N => Key::Ctrl('n'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_O => Key::Ctrl('o'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_P => Key::Ctrl('p'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_Q => Key::Ctrl('q'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_R => Key::Ctrl('r'),
            libcaca_sys::caca_key_CACA_KEY_PAUSE => Key::Pause,
            libcaca_sys::caca_key_CACA_KEY_CTRL_T => Key::Ctrl('t'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_U => Key::Ctrl('u'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_V => Key::Ctrl('v'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_W => Key::Ctrl('w'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_X => Key::Ctrl('x'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_Y => Key::Ctrl('y'),
            libcaca_sys::caca_key_CACA_KEY_CTRL_Z => Key::Ctrl('z'),
            libcaca_sys::caca_key_CACA_KEY_ESCAPE => Key::Escape,
            libcaca_sys::caca_key_CACA_KEY_DELETE => Key::Delete,
            libcaca_sys::caca_key_CACA_KEY_UP => Key::Up,
            libcaca_sys::caca_key_CACA_KEY_DOWN => Key::Down,
            libcaca_sys::caca_key_CACA_KEY_LEFT => Key::Left,
            libcaca_sys::caca_key_CACA_KEY_RIGHT => Key::Right,
            libcaca_sys::caca_key_CACA_KEY_INSERT => Key::Insert,
            libcaca_sys::caca_key_CACA_KEY_HOME => Key::Home,
            libcaca_sys::caca_key_CACA_KEY_END => Key::End,
            libcaca_sys::caca_key_CACA_KEY_PAGEUP => Key::PageUp,
            libcaca_sys::caca_key_CACA_KEY_PAGEDOWN => Key::PageDown,
            libcaca_sys::caca_key_CACA_KEY_F1 => Key::F(1),
            libcaca_sys::caca_key_CACA_KEY_F2 => Key::F(2),
            libcaca_sys::caca_key_CACA_KEY_F3 => Key::F(3),
            libcaca_sys::caca_key_CACA_KEY_F4 => Key::F(4),
            libcaca_sys::caca_key_CACA_KEY_F5 => Key::F(5),
            libcaca_sys::caca_key_CACA_KEY_F6 => Key::F(6),
            libcaca_sys::caca_key_CACA_KEY_F7 => Key::F(7),
            libcaca_sys::caca_key_CACA_KEY_F8 => Key::F(8),
            libcaca_sys::caca_key_CACA_KEY_F9 => Key::F(9),
            libcaca_sys::caca_key_CACA_KEY_F10 => Key::F(10),
            libcaca_sys::caca_key_CACA_KEY_F11 => Key::F(11),
            libcaca_sys::caca_key_CACA_KEY_F12 => Key::F(12),
            libcaca_sys::caca_key_CACA_KEY_F13 => Key::F(13),
            libcaca_sys::caca_key_CACA_KEY_F14 => Key::F(14),
            libcaca_sys::caca_key_CACA_KEY_F15 => Key::F(15),
            code => match char::from_u32(code) {
                Some(chr) => Key::Char(chr),
                None => Key::Unknown(raw_code),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Unknown(i32),
}

impl From<i32> for MouseButton {
    fn from(raw: i32) -> Self {
        match raw {
            1 => MouseButton::Left,
            2 => MouseButton::Right,
            3 => MouseButton::Middle,
            _ => MouseButton::Unknown(raw),
        }
    }
}

impl From<*const caca_event> for Event {
    #[allow(non_upper_case_globals)]
    fn from(raw: *const caca_event) -> Event {
        match unsafe { caca_get_event_type(raw) } {
            caca_event_type_CACA_EVENT_KEY_PRESS => parse_key_press(raw),
            caca_event_type_CACA_EVENT_KEY_RELEASE => parse_key_release(raw),
            caca_event_type_CACA_EVENT_MOUSE_PRESS => parse_mouse_press(raw),
            caca_event_type_CACA_EVENT_MOUSE_RELEASE => parse_mouse_release(raw),
            caca_event_type_CACA_EVENT_MOUSE_MOTION => parse_mouse_motion(raw),
            caca_event_type_CACA_EVENT_RESIZE => parse_resize(raw),
            caca_event_type_CACA_EVENT_QUIT => Event::Quit,
            _ => Event::None,
        }
    }
}

impl From<&caca_event> for Event {
    fn from(raw: &caca_event) -> Event {
        (raw as *const caca_event).into()
    }
}

impl From<caca_event> for Event {
    fn from(raw: caca_event) -> Event {
        (&raw).into()
    }
}

fn parse_key(raw: *const caca_event) -> (Key, u32, Option<String>) {
    let ch = unsafe { caca_get_event_key_ch(raw) };
    let utf32 = unsafe { caca_get_event_key_utf32(raw) };
    let mut utf8 = [0u8; 7];

    unsafe { caca_get_event_key_utf8(raw, (&mut utf8).as_ptr() as *mut _) };

    let utf8 = CStr::from_bytes_with_nul(&utf8[..])
        .map(|string| string.to_string_lossy().to_string())
        .unwrap_or_default();

    let utf8 = if utf8.is_empty() { None } else { Some(utf8) };

    (ch.into(), utf32, utf8)
}

#[inline]
fn parse_key_press(raw: *const caca_event) -> Event {
    let (k, utf32, utf8) = parse_key(raw);
    Event::KeyPress(k, utf32, utf8)
}

#[inline]
fn parse_key_release(raw: *const caca_event) -> Event {
    let (k, utf32, utf8) = parse_key(raw);
    Event::KeyRelease(k, utf32, utf8)
}

fn parse_mouse_button(raw: *const caca_event) -> MouseButton {
    unsafe { caca_get_event_mouse_button(raw) }.into()
}

#[inline]
fn parse_mouse_press(raw: *const caca_event) -> Event {
    Event::MousePress(parse_mouse_button(raw))
}

#[inline]
fn parse_mouse_release(raw: *const caca_event) -> Event {
    Event::MouseRelease(parse_mouse_button(raw))
}

fn parse_mouse_motion(raw: *const caca_event) -> Event {
    Event::MouseMotion(Point {
        x: unsafe { caca_get_event_mouse_x(raw) },
        y: unsafe { caca_get_event_mouse_y(raw) },
    })
}

fn parse_resize(raw: *const caca_event) -> Event {
    Event::Resize(Boundaries {
        width: unsafe { caca_get_event_resize_width(raw) as usize },
        height: unsafe { caca_get_event_resize_height(raw) as usize },
    })
}
