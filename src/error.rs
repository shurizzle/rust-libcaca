use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("not enough memory")]
    NotEnoughMemory,
    #[error("failed to open graphics device")]
    FailedToOpenGraphicsDevice,
    #[error("invalid size")]
    InvalidSize,
    #[error("invalid mask size")]
    InvalidMaskSize,
    #[error("canvas in use")]
    CanvasInUse,
    #[error("invalid refresh delay")]
    InvalidRefreshDelay,
    #[error("windows title unsupported")]
    WindowTitleUnsupported,
    #[error("mouse pointer unsupported")]
    MousePointerUnsupported,
    #[error("mouse cursor unsupported")]
    MouseCursorUnsupported,
    #[error("invalid dither params")]
    InvalidDitherParams,
    #[error("invalid brightness")]
    InvalidBrightness,
    #[error("invalid gamma")]
    InvalidGamma,
    #[error("invalid contrast")]
    InvalidContrast,
    #[error("invalid frame index")]
    InvalidFrameIndex,
    #[error("invalid color")]
    InvalidColor,
    #[error("request index is out of bounds")]
    OutOfBounds,
    #[error("an IO error occurred")]
    IO(#[from] std::io::Error),
    #[error("invalid FIGfont")]
    InvalidFIGfont,
    #[error("unknonw error")]
    Unknown(i32),
}
