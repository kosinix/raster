//!  A module for error types.

// from rust
use std::io::Error as IoError;
use std::num::ParseIntError;

// from external crates
use gif;
use piston_image::ImageError;
use png;

// from local crate

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

    GifDecode(gif::DecodingError),
    JpegDecode(ImageError),
    JpegEncode(ImageError),
    PngDecode(png::DecodingError),
    PngEncode(png::EncodingError),
    UnsupportedFormat(String),
    Unexpected,
}

/// Convert std::io::Error to RasterError::Io
impl From<IoError> for RasterError {
    fn from(err: IoError) -> RasterError {
        RasterError::Io(err)
    }
}

// GIF
/// Convert gif::DecodingError to RasterError::GifDecode
impl From<gif::DecodingError> for RasterError {
    fn from(err: gif::DecodingError) -> RasterError {
        RasterError::GifDecode(err)
    }
}

/// Convert png::DecodingError to RasterError::PngDecode
impl From<png::DecodingError> for RasterError {
    fn from(err: png::DecodingError) -> RasterError {
        RasterError::PngDecode(err)
    }
}

/// Convert png::EncodingError to RasterError::PngEncode
impl From<png::EncodingError> for RasterError {
    fn from(err: png::EncodingError) -> RasterError {
        RasterError::PngEncode(err)
    }
}

/// [Type alias](https://doc.rust-lang.org/book/error-handling.html#the-result-type-alias-idiom) for Result. 
pub type RasterResult<T> = Result<T, RasterError>;
