//!  A module for error types.

use std::io::Error as IoError;
use std::num::ParseIntError;
use image::ImageError;

/// Enumeration of raster's errors.
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
    Image(ImageError)
}

/// [Type alias](https://doc.rust-lang.org/book/error-handling.html#the-result-type-alias-idiom) for Result. 
pub type RasterResult<T> = Result<T, RasterError>;
