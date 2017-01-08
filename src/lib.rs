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
mod position;


// crates
extern crate image;

// from rust
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use std::ascii::AsciiExt;

// from external crate
use self::image::GenericImage;

// from local crate
use error::{RasterError, RasterResult};

// re-exports
pub use blend::BlendMode;
pub use editor::ResizeMode;
pub use filter::BlurMode;
pub use interpolate::InterpolationMode;
pub use position::PositionMode;
pub use transform::TransformMode;
pub use color::Color;

/// Holds histogram information.
pub type Histogram = (HashMap<u8, u32>, HashMap<u8, u32>, HashMap<u8, u32>, HashMap<u8, u32>);

/// Create an image from an image file.
///
/// # Errors
///
/// At present, this function relies on [image](https://github.com/PistonDevelopers/image), and
/// thus returns `RasterError::Image` upon failure.
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
            let src = try!(image::open(image_file).map_err(RasterError::Image)); // Returns image::DynamicImage
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
/// If writing to a file fails, this function returns `RasterError::Io`.
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
            image::save_buffer(
                &path,
                &image.bytes,
                image.width as u32,
                image.height as u32,
                image::RGBA(8)
            ).map_err(RasterError::Io)
        },
        "png"  => {
            Ok(try!(endec::encode_png(&image, &path)))
        },
        _ => {
            Err(RasterError::UnsupportedFormat(ext))
        }
    } 
}

/// A struct for easily representing a raster image.
#[derive(Debug, Clone)]
pub struct Image {
    /// Width of image in pixels.
    pub width: i32, //  i32 type is used as computation with negative integers is common.

    /// Height of image in pixels.
    pub height: i32,

    /// Vector containing sequence of bytes in RGBA format.
    pub bytes: Vec<u8>,
}

impl<'a> Image {

    /// Create a blank image. Default color is black.
    ///
    /// # Examples
    ///
    /// ```
    /// use raster::Image;
    ///
    /// let image = Image::blank(2, 2);
    ///
    /// println!("{:?}", image.bytes);
    ///
    /// assert_eq!(image.width, 2);
    /// assert_eq!(image.height, 2);
    /// ```
    pub fn blank(w:i32, h:i32) -> Image {

        let mut bytes = Vec::with_capacity((w * h) as usize * 4);
        for _ in 0..h {
            for _ in 0..w {
                bytes.extend_from_slice(&[0, 0, 0, 255]);
            }
        }
        Image {
            width: w,
            height: h,
            bytes: bytes
        }
    }

    /// Check if there is a pixel at this location given by x and y.
    ///
    /// # Examples
    ///
    /// ```
    /// use raster::Image;
    ///
    /// let image = Image::blank(2, 2);
    ///
    /// assert_eq!(image.check_pixel(0, 0), true);
    /// assert_eq!(image.check_pixel(3, 3), false);
    /// ```
    pub fn check_pixel(&self, x: i32, y:i32) -> bool {
        if y < 0 || y > self.height { // TODO: check on actual vectors and not just width and height?
            false
        } else {
            !(x < 0 || x > self.width)
        }
    }

    /// Get the histogram of the image.
    ///
    /// # Examples
    ///
    /// Visualizing the histogram of the red channel of this image:
    ///
    /// Image:
    ///
    /// ![](https://kosinix.github.io/raster/in/sample.png)
    ///
    /// Code:
    ///
    /// ```
    /// use raster::Image;
    /// use raster::Color;
    ///
    /// let image = raster::open("tests/in/sample.png").unwrap();
    ///
    /// let (r_bin, _, _, _) = image.histogram().unwrap();
    ///
    /// let mut max_r_bin = 0;
    /// for (_, count) in &r_bin {
    ///     if *count > max_r_bin {
    ///         max_r_bin = *count;
    ///     }
    /// }
    ///
    /// let canvas_w = 256;
    /// let canvas_h: i32 = 100;
    /// let mut image = Image::blank(canvas_w, canvas_h);
    /// raster::editor::fill(&mut image, Color::rgb(214, 214, 214)).unwrap();
    ///
    /// for x in 0..256 as i32 { // 0-255
    ///     let key = x as u8;
    ///     match r_bin.get(&key) {
    ///         Some(count) => {
    ///
    ///             let height = (canvas_h as f32 * (*count as f32 / max_r_bin as f32)).round() as i32;
    ///
    ///             for y in canvas_h-height..canvas_h {
    ///
    ///                 image.set_pixel(x, y, Color::hex("#e22d11").unwrap()).unwrap();
    ///
    ///             }
    ///         },
    ///         None => {}
    ///     }
    /// }
    ///
    /// raster::save(&image, "tests/out/histogram.png");
    /// ```
    ///
    /// Histogram:
    ///
    /// ![](https://kosinix.github.io/raster/out/histogram.png)
    ///
    /// Photoshop's result:
    ///
    /// ![](https://kosinix.github.io/raster/in/histogram-ps.png)
    ///
    pub fn histogram(&self) -> RasterResult<Histogram> {
        let w = self.width;
        let h = self.height;

        let mut r_bin: HashMap<u8, u32> = HashMap::new();
        let mut g_bin: HashMap<u8, u32> = HashMap::new();
        let mut b_bin: HashMap<u8, u32> = HashMap::new();
        let mut a_bin: HashMap<u8, u32> = HashMap::new();
        for y in 0..h {
            for x in 0..w {
                let pixel = try!(self.get_pixel(x, y));

                let r_bin_c = r_bin.entry(pixel.r).or_insert(0); // Insert the key with a value of 0 if key does not exist yet. Then return the count (which is zero).
                *r_bin_c += 1; // +1 to the count.

                let g_bin_c = g_bin.entry(pixel.g).or_insert(0);
                *g_bin_c += 1;

                let b_bin_c = b_bin.entry(pixel.b).or_insert(0);
                *b_bin_c += 1;

                let a_bin_c = a_bin.entry(pixel.a).or_insert(0);
                *a_bin_c += 1;

            }
        }

        Ok((r_bin, g_bin, b_bin, a_bin))
    }

    /// Get pixel in a given x and y location of an image.
    ///
    /// # Errors
    ///
    /// If either the x or y coordinate falls out of bounds, this will fail with
    /// `RasterError::PixelOutOfBounds`.
    ///
    /// # Examples
    ///
    /// ```
    /// use raster::Image;
    /// use raster::Color;
    ///
    /// let mut image = Image::blank(2, 2); // Creates a 2x2 black image.
    ///
    /// let pixel = image.get_pixel(0, 0).unwrap();
    ///
    /// assert_eq!(0, pixel.r);
    /// assert_eq!(0, pixel.g);
    /// assert_eq!(0, pixel.b);
    /// assert_eq!(255, pixel.a);
    /// ```
    pub fn get_pixel(&self, x: i32, y:i32) -> RasterResult<Color> {
        let rgba = 4;
        let start = (y * self.width) + x;
        let start = start * rgba;
        let end = start + rgba;
        let len = self.bytes.len();

        if start as usize > len || end as usize > len {
            Err(RasterError::PixelOutOfBounds(x, y))
        } else {
            let slice = &self.bytes[start as usize..end as usize];
            Ok(Color {
                r: slice[0],
                g: slice[1],
                b: slice[2],
                a: slice[3],
            })
        }
    }

    /// Set pixel in a given x and y location of an image.
    ///
    /// # Errors
    ///
    /// If either the x or y coordinate falls out of bounds, this will fail with
    /// `RasterError::PixelOutOfBounds`.
    ///
    /// If the calculated byte start index is less than 0, this will fail with
    /// `RasterError::InvalidStartIndex`.
    ///
    /// # Examples
    ///
    /// ```
    /// use raster::Image;
    /// use raster::Color;
    ///
    /// let mut image = Image::blank(2, 2); // Creates a 2x2 black image.
    ///
    /// let _ = image.set_pixel(0, 0, Color::rgba(255, 0, 0, 255)); // Set first pixel to red
    ///
    /// let pixel = image.get_pixel(0, 0).unwrap();
    ///
    /// assert_eq!(255, pixel.r);
    /// assert_eq!(0, pixel.g);
    /// assert_eq!(0, pixel.b);
    /// assert_eq!(255, pixel.a);
    /// ```
    pub fn set_pixel(&mut self, x: i32, y:i32, color: Color ) -> RasterResult<()> {
        let rgba = 4; // length
        let start = (y * &self.width) + x;
        let start = start * rgba;

        if x >= self.width || y >= self.height {
            Err(RasterError::PixelOutOfBounds(x, y))
        } else if start < 0 {
            Err(RasterError::InvalidStartIndex(start))
        } else {
            self.bytes[start as usize] = color.r;
            self.bytes[start as usize + 1] = color.g;
            self.bytes[start as usize + 2] = color.b;
            self.bytes[start as usize + 3] = color.a;

            Ok(())
        }
    }
}