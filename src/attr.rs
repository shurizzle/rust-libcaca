use libcaca_sys::{
    caca_attr_to_ansi, caca_attr_to_ansi_bg, caca_attr_to_ansi_fg, caca_attr_to_argb64,
    caca_attr_to_rgb12_bg, caca_attr_to_rgb12_fg,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Attr(u32);

impl From<u32> for Attr {
    fn from(arg: u32) -> Self {
        Attr(arg)
    }
}

impl Into<u32> for Attr {
    fn into(self) -> u32 {
        self.0
    }
}

impl Attr {
    pub fn ansi(&self) -> u8 {
        unsafe { caca_attr_to_ansi(self.0) }
    }

    pub fn ansi_fg(&self) -> u8 {
        unsafe { caca_attr_to_ansi_fg(self.0) }
    }

    pub fn ansi_bg(&self) -> u8 {
        unsafe { caca_attr_to_ansi_bg(self.0) }
    }

    pub fn rgb12_fg(&self) -> u16 {
        unsafe { caca_attr_to_rgb12_fg(self.0) }
    }

    pub fn rgb12_bg(&self) -> u16 {
        unsafe { caca_attr_to_rgb12_bg(self.0) }
    }

    pub fn argb64(&self) -> Argb {
        let mut res = Argb::default();

        unsafe { caca_attr_to_argb64(self.0, &mut res as *mut _ as *mut u8) };

        res
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Argb {
    pub bg_a: u8,
    pub bg_r: u8,
    pub bg_g: u8,
    pub bg_b: u8,
    pub fg_a: u8,
    pub fg_r: u8,
    pub fg_g: u8,
    pub fg_b: u8,
}

impl From<Attr> for Argb {
    fn from(attr: Attr) -> Self {
        attr.argb64()
    }
}
