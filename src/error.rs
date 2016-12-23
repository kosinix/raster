//!  A module for error types.

use std::io::Error as IoError;
use std::num::ParseIntError;
use image::ImageError;

#[derive(Debug)]
pub enum RasterError {
    Io(IoError),
    PixelOutOfBounds(i32, i32),
    InvalidStartIndex(i32),
    InvalidHex,
    HexParse(ParseIntError),
    BlendingImageFallsOutsideCanvas,
    InvalidGamma(f32),

    /*
    In an ideal world, image's error type needn't be exposed
    (but we don't live in an ideal world yet)
    */
    Image(ImageError),

    /*
    All of these invalid mode/type errors will be unneeded once mode/type flags are switched
    to using enums rather than strings.
    */
    InvalidResiveMode(String),
    InvalidBlurMode(String),
    InvalidInterpolationMode(String),
    InvalidTransformMode(String),
    InvalidPositionType(String)
}

pub type RasterResult<T> = Result<T, RasterError>;
