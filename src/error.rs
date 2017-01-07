//!  A module for error types.

use std::io::Error as IoError;
use std::num::ParseIntError;
use image::ImageError;
use png;
use gif;

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
    Image(ImageError),
    GifDecoding(gif::DecodingError),
    PngDecoding(png::DecodingError),
    PngEncoding(png::EncodingError),
    UnsupportedFormat(String),
}

/// Convert std::io::Error to RasterError::Io
impl From<IoError> for RasterError {
    fn from(err: IoError) -> RasterError {
        RasterError::Io(err)
    }
}

// GIF
/// Convert gif::DecodingError to RasterError::GifDecoding
impl From<gif::DecodingError> for RasterError {
    fn from(err: gif::DecodingError) -> RasterError {
        RasterError::GifDecoding(err)
    }
}

/// Convert png::DecodingError to RasterError::PngDecoding
impl From<png::DecodingError> for RasterError {
    fn from(err: png::DecodingError) -> RasterError {
        RasterError::PngDecoding(err)
    }
}

/// Convert png::EncodingError to RasterError::PngEncoding
impl From<png::EncodingError> for RasterError {
    fn from(err: png::EncodingError) -> RasterError {
        RasterError::PngEncoding(err)
    }
}

/// [Type alias](https://doc.rust-lang.org/book/error-handling.html#the-result-type-alias-idiom) for Result. 
pub type RasterResult<T> = Result<T, RasterError>;
