//! # Raster
//!
//! Raster is an image processing lib for Rust.
//!
//! It provides a simplified API for processing raster images (JPEG, PNG and GIF).
//!
//! ## Installation
//! Add this to your Cargo.toml file:
//!
//! ```rust,ignore
//! [dependencies]
//!
//! raster = "x.x.x"
//! ```
//! Where x are version numbers of the [latest version](https://crates.io/crates/raster) of raster. Eg.: 0.1.0
//!
//! Then add the raster crate in your main.rs:
//!
//! ```rust,ignore
//! extern crate raster; // In your main rust file
//! ```
//!
//! ## Creating Images
//! ### From an image file
//!
//! ```rust,ignore
//! // Create an image from file
//! let image = raster::open("tests/in/sample.png").unwrap();
//!
//! ```
//! Raster will detect the image format based on the file name.
//!
//! ### Create a blank image
//! ```rust,ignore
//! use raster::Image; // Include the Image struct
//!
//! // Create a blank 150x100 image. Defaults to a black background.
//! let image = Image::blank(150, 100);
//!
//! ```
//!
//! ## Saving Images
//! Save the opened image file:
//!
//! ```
//! // Create an image from file
//! let image = raster::open("tests/in/sample.png").unwrap();
//!
//! // Save opened image
//! raster::save(&image, "tests/out/test_open_save.png");
//!
//! ```
//!
//!
//!
//!
//! ## Blending 2 Images
//!
//! Here are two images blended using the normal mode.
//!
//! ![](https://kosinix.github.io/raster/out/test_blend_normal.png)
//!
//! More blending modes and options are available, see the blend API.
//!
//! ## Resizing Images
//!
//! An example of images resized to "fit" in a 200x200 box.
//!
//! ![](https://kosinix.github.io/raster/out/test_resize_fit_1.jpg) ![](https://kosinix.github.io/raster/out/test_resize_fit_2.jpg)
//!
//! More modes available, see the resize API.
//!
//! ## Rotating Images
//!
//! Images can be rotated both clockwise and counter-clockwise at any arbitrary angle with a custom background color.
//!
//! ![](https://kosinix.github.io/raster/out/test_transform_rotate_45.png)
//! ![](https://kosinix.github.io/raster/out/test_transform_rotate_45cc.png)
//!
//! ## And Many More...
//!
//! More options are available, checkout the modules below.
//!

// modules
pub mod compare;
pub mod editor;
pub mod error;
pub mod filter;
pub mod interpolate;
pub mod transform;
mod blend;
mod color;
mod endec;
mod image;
mod position;

// crates
extern crate gif;
extern crate image as piston_image;
extern crate png;

// from rust
use std::ascii::AsciiExt;
use std::fs::File;
use std::path::Path;

// from external crate
use piston_image::GenericImage;

// from local crate
use error::{RasterError, RasterResult};

// re-exports
pub use blend::BlendMode;
pub use color::Color;
pub use editor::ResizeMode;
pub use filter::BlurMode;
pub use image::Histogram;
pub use image::Image;
pub use image::ImageFormat;
pub use interpolate::InterpolationMode;
pub use position::PositionMode;
pub use transform::TransformMode;


/// Create an image from an image file.
///
/// # Errors
///
/// This function can return `RasterError::Io`, `RasterError::Decode`, or `RasterError::UnsupportedFormat` upon failure. 
/// See error module for more info.
///
/// # Examples
///
/// ```
/// // Create an image from file
/// let image = raster::open("tests/in/sample.png").unwrap();
/// println!("{:?}", image.bytes);
/// ```
pub fn open(image_file: &str) -> RasterResult<Image> {

    let path = Path::new(image_file);
    let ext = path.extension().and_then(|s| s.to_str())
                  .map_or("".to_string(), |s| s.to_ascii_lowercase());

    // Open the file with basic error check
    let file = try!(File::open(image_file));

    match &ext[..] {
        "gif"  => {
            Ok(try!(endec::decode_gif(&file)))
        },
        "jpg" | "jpeg" => {
            let src = try!(piston_image::open(image_file));
            let (w, h) = src.dimensions();
            let mut bytes = Vec::with_capacity((w * h) as usize * 4);
            for y in 0..h {
                for x in 0..w {
                    let p = src.get_pixel(x, y);
                    bytes.extend_from_slice(&p.data[0..4]);
                }
            }
            Ok(Image{
                width: w as i32,
                height: h as i32,
                bytes: bytes
            })
        },
        "png"  => {
            Ok(try!(endec::decode_png(&file)))
        },
        _ => {
            Err(RasterError::UnsupportedFormat(ext))
        }
    } 
}

/// Save an image to an image file. The image type is detected from the file extension of the file name.
///
/// # Errors
///
/// This function can return `RasterError::Io`, `RasterError::Encode`, or `RasterError::UnsupportedFormat` upon failure. 
/// See error module for more info.
///
/// # Examples
///
/// ```
/// // Create an image from file
/// let image = raster::open("tests/in/sample.png").unwrap();
/// raster::save(&image, "tests/out/test.png");
/// ```
pub fn save(image: &Image, out: &str) -> RasterResult<()> {

    let path = Path::new(out);
    let ext = path.extension().and_then(|s| s.to_str())
                  .map_or("".to_string(), |s| s.to_ascii_lowercase());

    match &ext[..] {
        "gif"  => {
            Ok(try!(endec::encode_gif(&image, &path)))
        },
        "jpg" | "jpeg" => {
            piston_image::save_buffer(
                &path,
                &image.bytes,
                image.width as u32,
                image.height as u32,
                piston_image::RGBA(8)
            ).map_err(|_| RasterError::Encode(ImageFormat::Jpeg, "Format".to_string()))
        },
        "png"  => {
            Ok(try!(endec::encode_png(&image, &path)))
        },
        _ => {
            Err(RasterError::UnsupportedFormat(ext))
        }
    } 
}