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
    /// File system read/write errors.
    Io(IoError),
    /// Getting or setting pixels outside of image bounds.
    PixelOutOfBounds(i32, i32),
    /// Invalid start index.
    InvalidStartIndex(i32),
    /// Hex format not supported.
    InvalidHex,
    /// Error parsing a hex string.
    HexParse(ParseIntError),
    /// Blending error.
    BlendingImageFallsOutsideCanvas,
    /// Invalid gamma parameter.
    InvalidGamma(f32),
    /// Error during GIF decoding.
    GifDecode(gif::DecodingError),
    /// Error during GIF encoding.
    GifEncode(gif::DecodingError), // TODO: Currently unused. And gif::EncodingError does not exist in gif crate.
    /// Error during JPEG decoding.
    JpegDecode(ImageError),
    /// Error during JPEG encoding.
    JpegEncode(ImageError),
    /// Error during PNG decoding.
    PngDecode(png::DecodingError),
    /// Error during PNG encoding.
    PngEncode(png::EncodingError),
    /// Unsupported image format.
    UnsupportedFormat(String),
    /// Error that does not belong in other variants.
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
