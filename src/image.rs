//!  A module for handling a raster image.

// crates
extern crate image;

// from rust


// from external crate
use self::image::GenericImage;

// from local crate
use color::Color;

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
    /// use raster::image::Image;
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
    /// use raster::image::Image;
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
    /// use raster::image::Image;
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
    /// use raster::image::Image;
    /// use raster::color::Color;
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
            return Err("Getting a pixel that is out of bounds.".to_string());
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
    /// use raster::image::Image;
    /// use raster::color::Color;
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
            return Err("Setting a pixel that is out of bounds.".to_string());
        }

        self.bytes[start as usize] = color.r;
        self.bytes[start as usize + 1] = color.g;
        self.bytes[start as usize + 2] = color.b;
        self.bytes[start as usize + 3] = color.a;

        Ok(())
    }
}