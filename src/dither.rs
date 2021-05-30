use std::{borrow::Borrow, ffi::CStr, str::FromStr};

use errno::errno;
use libcaca_sys::{
    caca_create_dither, caca_dither_t, caca_free_dither, caca_get_dither_algorithm,
    caca_get_dither_antialias, caca_get_dither_brightness, caca_get_dither_charset,
    caca_get_dither_color, caca_get_dither_contrast, caca_get_dither_gamma,
    caca_set_dither_algorithm, caca_set_dither_antialias, caca_set_dither_brightness,
    caca_set_dither_charset, caca_set_dither_color, caca_set_dither_contrast,
    caca_set_dither_gamma, caca_set_dither_palette,
};

use crate::{error::Error, result::Result, utils::lossy_cstring};

pub struct Dither(*mut caca_dither_t);

impl Dither {
    pub fn new(
        bpp: i32,
        width: i32,
        height: i32,
        pitch: i32,
        rmask: u32,
        gmask: u32,
        bmask: u32,
        amask: u32,
    ) -> Result<Dither> {
        let raw_dither =
            unsafe { caca_create_dither(bpp, width, height, pitch, rmask, gmask, bmask, amask) };

        if raw_dither.is_null() {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidDitherParams),
                libc::ENOMEM => Err(Error::NotEnoughMemory),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(Dither(raw_dither))
        }
    }

    pub fn set_palette(
        &self,
        red: &[u32],
        green: &[u32],
        blue: &[u32],
        alpha: &[u32],
    ) -> Result<()> {
        if unsafe {
            caca_set_dither_palette(
                self.as_internal(),
                red.as_ptr() as *mut _,
                green.as_ptr() as *mut _,
                blue.as_ptr() as *mut _,
                alpha.as_ptr() as *mut _,
            )
        } != 0
        {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidDitherParams),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn set_brightness(&self, brightness: f32) -> Result<()> {
        if unsafe { caca_set_dither_brightness(self.as_internal(), brightness) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidBrightness),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn brightness(&self) -> f32 {
        unsafe { caca_get_dither_brightness(self.as_internal()) }
    }

    pub fn set_gamma(&self, gamma: f32) -> Result<()> {
        if unsafe { caca_set_dither_gamma(self.as_internal(), gamma) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidGamma),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn gamma(&self) -> f32 {
        unsafe { caca_get_dither_gamma(self.as_internal()) }
    }

    pub fn set_contrast(&self, contrast: f32) -> Result<()> {
        if unsafe { caca_set_dither_contrast(self.as_internal(), contrast) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidContrast),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn contrast(&self) -> f32 {
        unsafe { caca_get_dither_contrast(self.as_internal()) }
    }

    pub fn set_antialias(&self, antialias: Antialias) -> Result<()> {
        let antialias = antialias.to_string();
        let antialias = lossy_cstring(antialias);
        if unsafe { caca_set_dither_antialias(self.as_internal(), antialias.as_ptr()) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidAntialias),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn antialias(&self) -> Result<Antialias> {
        let s = unsafe { CStr::from_ptr(caca_get_dither_antialias(self.as_internal())) }
            .to_string_lossy();
        Antialias::from_str(s.as_ref())
    }

    pub fn set_color(&self, color: DitherColor) -> Result<()> {
        let color = color.to_string();
        let color = lossy_cstring(color);
        if unsafe { caca_set_dither_color(self.as_internal(), color.as_ptr()) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidColor),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn color(&self) -> Result<DitherColor> {
        let s =
            unsafe { CStr::from_ptr(caca_get_dither_color(self.as_internal())) }.to_string_lossy();
        DitherColor::from_str(s.as_ref())
    }

    pub fn set_charset(&self, charset: DitherCharset) -> Result<()> {
        let charset = charset.to_string();
        let charset = lossy_cstring(charset);
        if unsafe { caca_set_dither_charset(self.as_internal(), charset.as_ptr()) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidCharset),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn charset(&self) -> Result<DitherCharset> {
        let s = unsafe { CStr::from_ptr(caca_get_dither_charset(self.as_internal())) }
            .to_string_lossy();
        DitherCharset::from_str(s.as_ref())
    }

    pub fn set_algorithm(&self, algorithm: DitherAlgorithm) -> Result<()> {
        let algorithm = algorithm.to_string();
        let algorithm = lossy_cstring(algorithm);
        if unsafe { caca_set_dither_algorithm(self.as_internal(), algorithm.as_ptr()) } != 0 {
            match errno().0 {
                libc::EINVAL => Err(Error::InvalidAlgorithm),
                what => Err(Error::Unknown(what)),
            }
        } else {
            Ok(())
        }
    }

    pub fn algorithm(&self) -> Result<DitherAlgorithm> {
        let s = unsafe { CStr::from_ptr(caca_get_dither_algorithm(self.as_internal())) }
            .to_string_lossy();
        DitherAlgorithm::from_str(s.as_ref())
    }

    pub(crate) fn as_internal(&self) -> *mut caca_dither_t {
        self.0
    }
}

impl Drop for Dither {
    fn drop(&mut self) {
        unsafe { caca_free_dither(self.as_internal()) };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Antialias {
    None,
    Prefilter,
}

impl Default for Antialias {
    fn default() -> Self {
        Self::Prefilter
    }
}

impl ToString for Antialias {
    fn to_string(&self) -> String {
        match self {
            Antialias::None => "none",
            Antialias::Prefilter => "prefilter",
        }
        .to_string()
    }
}

impl FromStr for Antialias {
    type Err = Error;

    fn from_str(raw: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        match raw.to_lowercase().borrow() {
            "none" => Ok(Self::None),
            "prefilter" => Ok(Self::Prefilter),
            "default" => Ok(Self::default()),
            _ => Err(Error::InvalidAntialias),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DitherColor {
    Mono,
    Gray,
    Height,
    Sixteen,
    FullGray,
    Full8,
    Full16,
}

impl Default for DitherColor {
    fn default() -> Self {
        Self::Full16
    }
}

impl ToString for DitherColor {
    fn to_string(&self) -> String {
        match self {
            Self::Mono => "mono",
            Self::Gray => "gray",
            Self::Height => "8",
            Self::Sixteen => "16",
            Self::FullGray => "fullgray",
            Self::Full8 => "full8",
            Self::Full16 => "full16",
        }
        .to_string()
    }
}

impl FromStr for DitherColor {
    type Err = Error;

    fn from_str(raw: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        match raw.to_lowercase().borrow() {
            "mono" => Ok(Self::Mono),
            "gray" => Ok(Self::Gray),
            "8" => Ok(Self::Height),
            "16" => Ok(Self::Sixteen),
            "fullgray" => Ok(Self::FullGray),
            "full8" => Ok(Self::Full8),
            "full16" => Ok(Self::Full16),
            "default" => Ok(Self::default()),
            _ => Err(Error::InvalidColor),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DitherCharset {
    Ascii,
    Shades,
    Blocks,
}

impl Default for DitherCharset {
    fn default() -> Self {
        Self::Ascii
    }
}

impl ToString for DitherCharset {
    fn to_string(&self) -> String {
        match self {
            Self::Ascii => "ascii",
            Self::Shades => "shades",
            Self::Blocks => "blocks",
        }
        .to_string()
    }
}

impl FromStr for DitherCharset {
    type Err = Error;
    fn from_str(raw: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        match raw.to_lowercase().borrow() {
            "ascii" => Ok(Self::Ascii),
            "shades" => Ok(Self::Shades),
            "blocks" => Ok(Self::Blocks),
            "default" => Ok(Self::default()),
            _ => Err(Error::InvalidCharset),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DitherAlgorithm {
    None,
    Ordered2,
    Ordered4,
    Ordered8,
    Random,
    Fstein,
}

impl ToString for DitherAlgorithm {
    fn to_string(&self) -> String {
        match self {
            Self::None => "none",
            Self::Ordered2 => "ordered2",
            Self::Ordered4 => "ordered4",
            Self::Ordered8 => "ordered8",
            Self::Random => "random",
            Self::Fstein => "fstein",
        }
        .to_string()
    }
}

impl FromStr for DitherAlgorithm {
    type Err = Error;

    fn from_str(raw: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        match raw.to_lowercase().borrow() {
            "none" => Ok(Self::None),
            "ordered2" => Ok(Self::Ordered2),
            "ordered4" => Ok(Self::Ordered4),
            "ordered8" => Ok(Self::Ordered8),
            "random" => Ok(Self::Random),
            "fstein" => Ok(Self::Fstein),
            _ => Err(Error::InvalidAlgorithm),
        }
    }
}
