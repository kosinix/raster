//!  A module for error types.

use std::convert::TryInto;
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
        if let piston_image::ImageError::Parameter(_) = err {
            return RasterError::Unexpected;
        }
        if let piston_image::ImageError::IoError(io_err) = err {
            return RasterError::Io(io_err);
        }
        if let piston_image::ImageError::Limits(limit_err) = err {
            match limit_err.kind() {
                piston_image::error::LimitErrorKind::DimensionError => {
                    return RasterError::Unexpected; // TODO: where to get dimensions from ?
                }
                piston_image::error::LimitErrorKind::InsufficientMemory => {
                    return RasterError::Io(std::io::Error::new(
                        std::io::ErrorKind::OutOfMemory,
                        "insufficient memory",
                    ))
                }
                piston_image::error::LimitErrorKind::Unsupported { limits, .. } => {
                    match (limits.max_image_width, limits.max_image_height) {
                        (Some(w), Some(h)) => {
                            return RasterError::PixelOutOfBounds(w as i32, h as i32)
                        }
                        _ => return RasterError::Unexpected, // TODO: where to get dimensions from ?
                    }
                }
                _ => return RasterError::Unexpected,
            }
        }
        let hint: piston_image::error::ImageFormatHint = match &err {
            piston_image::ImageError::Encoding(encoding_err) => encoding_err.format_hint(),
            piston_image::ImageError::Decoding(decoding_err) => decoding_err.format_hint(),
            _ => unreachable!(), // processed above
        };
        let format: Result<ImageFormat, RasterError> = match &err {
            piston_image::ImageError::Decoding(_) => hint.try_into(),
            piston_image::ImageError::Encoding(_) => hint.try_into(),
            _ => unreachable!(), // processed above
        };
        match (&err, format) {
            (_, Err(raster_err)) => raster_err,
            (piston_image::ImageError::Encoding(_), Ok(format)) => {
                return RasterError::Encode(format, err.to_string())
            }
            (piston_image::ImageError::Decoding(_), Ok(format)) => {
                return RasterError::Decode(format, err.to_string())
            }
            _ => unreachable!(), // processed above
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
            png::DecodingError::Parameter(_) => RasterError::Unexpected,
            png::DecodingError::LimitsExceeded => RasterError::Unexpected, // TODO: where to get dimensions from ?
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
            png::EncodingError::Parameter(_) => RasterError::Unexpected,
            png::EncodingError::LimitsExceeded => RasterError::Unexpected, // TODO: where to get dimensions from ?
        }
    }
}

/// [Type alias](https://doc.rust-lang.org/book/error-handling.html#the-result-type-alias-idiom)
/// for Result.
pub type RasterResult<T> = Result<T, RasterError>;
