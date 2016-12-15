//! # Raster
//!
//! Raster provides a simplified API for processing pixels of raster images.
//! 
//! ## Creating Images
//! ### From an image file
//! ```
//! // Create an image from file
//! let image = raster::open("tests/in/sample.png").unwrap();
//!
//! // Save opened image
//! raster::save(&image, "tests/out/test_open_save.png");
//!
//! ```
//!
//! ### Create a blank image
//! ```
//! use raster::Image;
//! 
//! // Create a blank 100x100 image. Defaults to a black background.
//! let image = Image::blank(100, 100);
//!
//! // Save blank
//! raster::save(&image, "tests/out/test_blank.png");
//!
//! ```
//! A blank image:
//!
//! ![Blank](https://kosinix.github.io/raster/out/test_blank.png)
//! 
//! ## Editing and Saving Images
//!
//! ```
//! use raster::Image;
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
//! use raster::Image;
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


pub mod compare;
pub mod editor;
pub mod filter;
pub mod interpolate;
pub mod transform;
mod blend;
mod position;

// crates
extern crate image;

// from rust

// from external crate
use self::image::GenericImage;

// from local crate


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

/// A struct for easily representing a raster image.
#[derive(Debug)]
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
        
        let mut bytes = Vec::new();
        for _ in 0..h {
            for _ in 0..w {
                bytes.push(0);
                bytes.push(0);
                bytes.push(0);
                bytes.push(255);
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
            return false;

        } else if x < 0 || x > self.width {
            return false;
        }

        true
    }

    /// Create an image from an image file.
    ///
    /// # Examples
    /// 
    /// ```
    /// use raster::Image;
    ///
    /// // Create an image from file
    /// let image = raster::open("tests/in/sample.png").unwrap();
    /// println!("{:?}", image.bytes);
    /// ```
    pub fn from_file(file: &'a str) -> Result<Image, String> {
        
        let src = image::open(file).unwrap(); // Returns image::DynamicImage
        let (w, h) = src.dimensions();
        let mut bytes = Vec::new();
        for y in 0..h {
            for x in 0..w {
                let p = src.get_pixel(x, y);
                bytes.push(p.data[0]);
                bytes.push(p.data[1]);
                bytes.push(p.data[2]);
                bytes.push(p.data[3]);
            }
        }
        Ok(Image{ 
            width: w as i32,
            height: h as i32,
            bytes: bytes
        })
    }

    /// Get pixel in a given x and y location of an image.
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
    pub fn get_pixel(&self, x: i32, y:i32) -> Result<Color, String> {
        let rgba = 4;
        let start = (y * &self.width) + x;
        let start = start * rgba;
        let end = start + rgba;
        let len = self.bytes.len();

        if start as usize > len || end as usize > len {
            return Err(format!("Getting a pixel at ({}, {}) that is out of bounds.", x, y).to_string());
        }
        
        let slice = &self.bytes[start as usize..end as usize];
        Ok(Color {
            r: slice[0],
            g: slice[1],
            b: slice[2],
            a: slice[3],
        })
    }

    /// Set pixel in a given x and y location of an image.
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
    pub fn set_pixel(&mut self, x: i32, y:i32, color: Color ) -> Result<(), String> {
        let rgba = 4; // length
        let start = (y * &self.width) + x;
        let start = start * rgba;
        
        if x >= self.width || y >= self.height {
            return Err(format!("Setting a pixel that is out of bounds at ({}, {}).", x, y).to_string());
        }

        self.bytes[start as usize] = color.r;
        self.bytes[start as usize + 1] = color.g;
        self.bytes[start as usize + 2] = color.b;
        self.bytes[start as usize + 3] = color.a;

        Ok(())
    }
}



/// A struct for representing and creating color.
#[derive(Debug)]
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

    /// Clones a Color.
    pub fn clone(&self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }

    /// Returns a black Color.
    pub fn black() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
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

    /// Returns a red Color.
    pub fn red() -> Color {
        Color {
            r: 255,
            g: 0,
            b: 0,
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

    /// Returns a white Color.
    pub fn blue() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }
}
