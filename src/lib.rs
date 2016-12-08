//! # Raster
//!
//! Raster provides a simplified API for processing pixels of raster images.
//! 
//! ## Creating Images
//! 
//! ```
//! use raster::image::Image;
//!
//! // Create an image from file
//! let image1 = raster::open("tests/in/sample.png").unwrap();
//! 
//! // Create a blank 100x100 image. Defaults to a black background.
//! let image2 = Image::blank(100, 100);
//!
//! // Save blank
//! raster::save(&image2, "tests/out/test_blank.png");
//!
//! ```
//! A blank image:
//!
//! ![Blank](https://kosinix.github.io/raster/out/test_blank.png)
//! 
//! ## Editing and Saving Images
//!
//! ```
//! use raster::image::Image;
//! use raster::editor;
//!
//! // Create an image from file
//! let image = raster::open("tests/in/sample.png").unwrap();
//! 
//! // Resize an image to exactly 200x200 pixels
//! let image = editor::resize_exact(&image, 200, 200).unwrap();
//!
//! // Save it
//! editor::save(&image, "tests/out/test_resize_exact.png");
//! ```
//! ![Resize exact](https://kosinix.github.io/raster/out/test_resize_exact.png)
//!
//! ## Blending 2 Images
//!
//! ```
//! use raster::image::Image;
//! use raster::editor;
//!
//! // Create images from file
//! let image1 = raster::open("tests/in/sample.jpg").unwrap();
//! let image2 = raster::open("tests/in/watermark.png").unwrap();
//! 
//! // Blend image2 on top of image1 using normal mode, opacity of 1.0 (100%), with image2 at the center, with 0 x and 0 y offsets. whew
//! let image3 = editor::blend(&image1, &image2, "normal", 1.0, "center", 0, 0).unwrap();
//!
//! // Save it
//! editor::save(&image3, "tests/out/test_blend_normal.png");
//! ```
//! ![Blend Normal](https://kosinix.github.io/raster/out/test_blend_normal.png)
//!
//! See the modules, more specifically the editor module for more info.
//!


pub mod color;
pub mod compare;
pub mod editor;
pub mod filter;
pub mod image;
pub mod interpolate;
pub mod transform;
mod blend;
mod position;

use image::Image;

/// Create an image from an image file. Returns raster::image::Image.
///
/// # Examples
/// 
/// ```
/// // Create an image from file
/// let image = raster::open("tests/in/sample.png").unwrap();
/// println!("{:?}", image.bytes);
/// ```
pub fn open(image_file: &str) -> Result<Image, String> {
    
    let image = try!(Image::from_file(image_file));

    Ok(image)
}

/// Save an image to an image file. The image type is detected from the file extension of the file name.
///
/// # Examples
/// 
/// ```
/// // Create an image from file
/// let image = raster::open("tests/in/sample.png").unwrap();
/// raster::save(&image, "tests/out/test.png");
/// ```
pub fn save(image: &Image, out: &str) {
    editor::save(&image, out);
}