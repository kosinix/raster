//!  A module for error types.

// from rust
use std::io::Error as IoError;
use std::num::ParseIntError;

// from external crates
use gif;
use piston_image;
use png;

// from local crate
use ImageFormat;

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
    /// Error during decoding.
    Decode(ImageFormat, String),
    /// Error during encoding.
    Encode(ImageFormat, String),
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
/// Convert gif::DecodingError to RasterError::Decode
impl From<gif::DecodingError> for RasterError {
    fn from(err: gif::DecodingError) -> RasterError {
        match err {
            gif::DecodingError::Format(msg) => {
                RasterError::Decode(ImageFormat::Gif, msg.to_string())
            }
            gif::DecodingError::Internal(msg) => {
                RasterError::Decode(ImageFormat::Gif, msg.to_string())
            }
            gif::DecodingError::Io(io_err) => RasterError::Io(io_err),
        }
    }
}
// NOTE: gif::EncodingError does not exist in gif crate.

// JPEG
/// Convert gif::DecodingError to RasterError::Decode
// NOTE: We assume that we are in decoding jpeg since this error's entry point is only in
// raster::open
impl From<piston_image::ImageError> for RasterError {
    fn from(err: piston_image::ImageError) -> RasterError {
        match err {
            piston_image::ImageError::FormatError(msg) => {
                RasterError::Decode(ImageFormat::Jpeg, msg)
            }
            piston_image::ImageError::DimensionError => {
                RasterError::Decode(ImageFormat::Jpeg, "DimensionError".to_string())
            }
            piston_image::ImageError::UnsupportedError(msg) => {
                RasterError::Decode(ImageFormat::Jpeg, msg)
            }
            piston_image::ImageError::UnsupportedColor(_) => {
                RasterError::Decode(ImageFormat::Jpeg, "UnsupportedColor".to_string())
            }
            piston_image::ImageError::NotEnoughData => {
                RasterError::Decode(ImageFormat::Jpeg, "NotEnoughData".to_string())
            }
            piston_image::ImageError::IoError(io_err) => RasterError::Io(io_err),
            piston_image::ImageError::ImageEnd => {
                RasterError::Decode(ImageFormat::Jpeg, "ImageEnd".to_string())
            }
        }
    }
}

// PNG
/// Convert png::DecodingError to RasterError::Decode
impl From<png::DecodingError> for RasterError {
    fn from(err: png::DecodingError) -> RasterError {
        match err {
            png::DecodingError::IoError(io_err) => RasterError::Io(io_err),
            png::DecodingError::Format(_) => {
                RasterError::Decode(ImageFormat::Png, "Format".to_string())
            }
            png::DecodingError::InvalidSignature => {
                RasterError::Decode(ImageFormat::Png, "InvalidSignature".to_string())
            }
            png::DecodingError::CrcMismatch { .. } => {
                RasterError::Decode(ImageFormat::Png, "CrcMismatch".to_string())
            }
            png::DecodingError::Other(_) => {
                RasterError::Decode(ImageFormat::Png, "Other".to_string())
            }
            png::DecodingError::CorruptFlateStream => {
                RasterError::Decode(ImageFormat::Png, "CorruptFlateStream".to_string())
            }
        }
    }
}

/// Convert png::EncodingError to RasterError::Encode
impl From<png::EncodingError> for RasterError {
    fn from(err: png::EncodingError) -> RasterError {
        match err {
            png::EncodingError::IoError(io_err) => RasterError::Io(io_err),
            png::EncodingError::Format(_) => {
                RasterError::Encode(ImageFormat::Png, "Format".to_string())
            }
        }
    }
}

/// [Type alias](https://doc.rust-lang.org/book/error-handling.html#the-result-type-alias-idiom)
/// for Result.
pub type RasterResult<T> = Result<T, RasterError>;
