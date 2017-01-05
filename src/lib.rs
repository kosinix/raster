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
pub mod error;
pub mod compare;
pub mod editor;
pub mod filter;
pub mod interpolate;
pub mod transform;
mod blend;
mod position;

// crates
extern crate image;
extern crate png;

// from rust
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::collections::HashMap;
use std::ascii::AsciiExt;

// from external crate
use self::image::GenericImage;
use png::HasParameters; // to use set()

// from local crate
use error::{RasterError, RasterResult};
pub use blend::BlendMode;
pub use editor::ResizeMode;
pub use filter::BlurMode;
pub use interpolate::InterpolationMode;
pub use position::PositionMode;
pub use transform::TransformMode;

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


    match &ext[..] {
        "jpg" | "jpeg" | "gif"  => {
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
            Ok(try!(_decode_png(image_file)))
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
        "jpg" | "jpeg" | "gif"  => {
            image::save_buffer(
                &Path::new(out),
                &image.bytes,
                image.width as u32,
                image.height as u32,
                image::RGBA(8)
            ).map_err(RasterError::Io)
        },
        "png"  => {
            Ok(try!(_encode_png(&image, &path)))
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

fn rgb_min(r: f32, g: f32, b: f32) -> f32 {
    let min = if g < r {
        g
    } else {
        r
    };

    if b < min {
        b
    } else {
        min
    }
}

fn rgb_max(r: f32, g: f32, b: f32) -> f32 {
    let max = if g > r {
        g
    } else {
        r
    };

    if b > max {
        b
    } else {
        max
    }
}

/// A struct for representing and creating color.
#[derive(Debug, Clone)]
pub struct Color {
    /// Red channel 0 - 255
    pub r: u8,

    /// Green channel 0 - 255
    pub g: u8,

    /// Blue channel 0 - 255
    pub b: u8,

    /// Alpha channel 0 - 255
    pub a: u8,
}

impl<'a> Color {

    /// Returns a black Color.
    pub fn black() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    /// Returns a blue Color.
    pub fn blue() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    /// Returns a green Color.
    pub fn green() -> Color {
        Color {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    /// Create a color from hexadecimal value.
    ///
    /// Example of valid formats: #FFFFFF, #ffeecc, #00ff007f
    ///
    /// # Errors
    ///
    /// If the hex *string* is malformed (doesn't begin with `#` or is of invalid length) then this
    /// fails with `RasterError::InvalidHex`. If it passes that, but the string can't be parsed
    /// into actual values, then this fails with `RasterError::HexParse`.
    ///
    /// # Examples
    /// ```
    /// use raster::Color;
    ///
    /// // Ok tests
    /// let color = Color::hex("#FFFFFF"); // Opaque white
    /// assert!(color.is_ok());
    ///
    /// let color = Color::hex("#00FF007F"); // Green with 50% opacity
    /// assert!(color.is_ok());
    ///
    /// // Error tests
    /// let color = Color::hex("");
    /// assert!(color.is_err());
    ///
    /// let color = Color::hex("#");
    /// assert!(color.is_err());
    ///
    /// let color = Color::hex("#FFF");
    /// assert!(color.is_err());
    ///
    /// ```
    ///
    /// To get the value, use unwrap:
    ///
    /// ```
    /// use raster::Color;
    ///
    /// let color = Color::hex("#00FF007F").unwrap();
    /// assert_eq!(255, color.g);
    /// ```
    pub fn hex(hex: &str) -> RasterResult<Color> {
        if hex.len() == 9 && hex.starts_with('#') { // #FFFFFFFF (Red Green Blue Alpha)
            Ok(Color {
                r: try!(_hex_dec(&hex[1..3])),
                g: try!(_hex_dec(&hex[3..5])),
                b: try!(_hex_dec(&hex[5..7])),
                a: try!(_hex_dec(&hex[7..9])),
            })
        } else if hex.len() == 7 && hex.starts_with('#') { // #FFFFFF (Red Green Blue)
            Ok(Color {
                r: try!(_hex_dec(&hex[1..3])),
                g: try!(_hex_dec(&hex[3..5])),
                b: try!(_hex_dec(&hex[5..7])),
                a: 255,
            })
        } else {
            Err(RasterError::InvalidHex)
        }
    }

    /// Returns a red Color.
    pub fn red() -> Color {
        Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    /// Create a RGB color. Alpha defaults to opaque (255).
    ///
    /// # Examples
    ///
    /// ```
    /// use raster::Color;
    ///
    /// let rgb = Color::rgb(0, 255, 0); // Green
    ///
    /// println!("{:?}", rgb);
    ///
    /// assert_eq!(rgb.r, 0);
    /// assert_eq!(rgb.g, 255);
    /// assert_eq!(rgb.b, 0);
    /// assert_eq!(rgb.a, 255);
    /// ```
    pub fn rgb(r:u8, g:u8, b:u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: 255,
        }
    }

    /// Create a RGBA color.
    ///
    /// # Examples
    ///
    /// ```
    /// use raster::Color;
    ///
    /// let rgba = Color::rgba(0, 0, 255, 255); // Blue
    ///
    /// println!("{:?}", rgba);
    ///
    /// assert_eq!(rgba.r, 0);
    /// assert_eq!(rgba.g, 0);
    /// assert_eq!(rgba.b, 255);
    /// assert_eq!(rgba.a, 255);
    /// ```
    pub fn rgba(r:u8, g:u8, b:u8, a:u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    /// Convert RGB to HSV/HSB (Hue, Saturation, Brightness).
    ///
    /// ```
    /// use raster::Color;
    ///
    /// let hsv = Color::to_hsv(50, 50, 100);
    ///
    /// assert_eq!(240, hsv.0);
    /// assert_eq!(50.0, (hsv.1).round()); // Saturation in float
    /// assert_eq!(39.0, (hsv.2).round()); // Brightness in float
    /// ```
    // Using f32 for s,v for accuracy when converting from RGB-HSV and vice-versa.
    pub fn to_hsv(r: u8, g: u8, b: u8) -> (u16, f32, f32) {

        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let min = rgb_min(r, g, b);
        let max = rgb_max(r, g, b);

        let chroma = max - min;

        let h = {
            let mut h = 0.0;

            if chroma != 0.0 {
                if (max - r).abs() < std::f32::EPSILON {
                    h = 60.0 * ((g - b) / chroma);
                    if h < 0.0 {
                        h += 360.0;
                    }
                } else if (max - g).abs() < std::f32::EPSILON {
                    h = 60.0 * (((b - r) / chroma) + 2.0);
                } else if (max - b).abs() < std::f32::EPSILON {
                    h = 60.0 * (((r - g) / chroma) + 4.0);
                }
            }

            if h > 359.0 {
                h = 360.0 - h; // Invert if > 0 to 359
            }

            h
        };

        let v = max;
        let s = if v != 0.0 {
            chroma / v
        } else {
            0.0
        };

        ( h.round() as u16, s * 100.0, v * 100.0  )
    }

    /// Convert HSV/HSB (Hue, Saturation, Brightness) to RGB.
    ///
    /// ```
    /// use raster::Color;
    ///
    /// let rgb1 = (127, 70, 60);
    /// let hsv = Color::to_hsv(rgb1.0, rgb1.1, rgb1.2); // Convert to HSV
    /// let rgb2 = Color::to_rgb(hsv.0, hsv.1, hsv.2); // Convert back to RGB
    ///
    /// // Check if source RGB is equal to final RGB
    /// assert_eq!(rgb1.0, rgb2.0);
    /// assert_eq!(rgb1.1, rgb2.1);
    /// assert_eq!(rgb1.2, rgb2.2);
    /// ```
    // Using f32 for s,v for accuracy when converting from RGB-HSV and vice-versa.
    pub fn to_rgb(h:u16, s: f32, v: f32) -> (u8, u8, u8) {

        let h = h as f32 / 60.0;
        let s = s as f32 / 100.0; // Convert to 0.0 - 1.0
        let v = v as f32 / 100.0;

        let chroma = v * s;

        let x = chroma * ( 1.0 - ( (h % 2.0) - 1.0 ).abs() );

        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        if h >= 0.0 {
            if h < 1.0 {
                r = chroma;
                g = x;
                b = 0.0;
            } else if h < 2.0 {
                r = x;
                g = chroma;
                b = 0.0;
            } else if h < 3.0 {
                r = 0.0;
                g = chroma;
                b = x;
            } else if h < 4.0 {
                r = 0.0;
                g = x;
                b = chroma;
            } else if h < 5.0 {
                r = x;
                g = 0.0;
                b = chroma;
            } else if h < 6.0 {
                r = chroma;
                g = 0.0;
                b = x;
            }
        }

        let m = v - chroma;
        r += m;
        g += m;
        b += m;
        ( (r * 255.0).round() as u8, (g * 255.0).round() as u8, (b * 255.0).round() as u8)
    }

    /// Returns a white Color.
    pub fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

// Private functions

// Convert a hex string to decimal. Eg. "00" -> 0. "FF" -> 255.
fn _hex_dec(hex_string: &str) -> RasterResult<u8> {
    u8::from_str_radix(hex_string, 16)
        .map(|o| o as u8)
        .map_err(RasterError::HexParse)
}

// Decode PNG
fn _decode_png(image_file: &str) -> RasterResult<Image>{
    let f = try!(File::open(image_file));
    let decoder = png::Decoder::new(f);
    let (info, mut reader) = try!(decoder.read_info());
    let mut bytes = vec![0; info.buffer_size()];
    
    try!(reader.next_frame(&mut bytes));

    if info.color_type == png::ColorType::RGB { // Applies only to RGB

        let mut insert_count = 0;
        let len = (info.width * info.height) as usize;
        for i in 0..len {
            let insert_pos = 3 * (i+1) + insert_count;
            bytes.insert(insert_pos, 255);
            insert_count+=1;
        }
    } //  TODO other ::ColorType
    Ok(
        Image {
            width: info.width as i32,
            height: info.height as i32,
            bytes: bytes
        }
    )
}

// Encode PNG
fn _encode_png(image: &Image, path: &Path) -> RasterResult<()>{
    let file = try!(File::create(path));
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image.width as u32, image.height as u32);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = try!(encoder.write_header());
    Ok(try!(writer.write_image_data(&image.bytes)))
}