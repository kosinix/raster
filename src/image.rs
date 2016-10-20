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
    pub pixels: Vec<u8>,
}

impl<'a> Image {
    
    /// Create a blank image.
    ///
    /// # Examples
    ///
    /// ```
    /// use raster::image::Image;
    ///
    /// let image = Image::blank(2, 2);
    /// 
    /// println!("{:?}", image.pixels);
    ///
    /// assert_eq!(image.width, 2);
    /// assert_eq!(image.height, 2);
    /// ```
    pub fn blank(w:i32, h:i32) -> Image {
        
        let mut pixels = Vec::new();
        for _ in 0..h {
            for _ in 0..w {
                pixels.push(0);
                pixels.push(0);
                pixels.push(0);
                pixels.push(255);
            }
        }
        Image { 
            width: w,
            height: h,
            pixels: pixels
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
    /// let image = Image::from_file("tests/image/sample.png");
    /// println!("{:?}", image.pixels);
    /// ```
    pub fn from_file(file: &'a str) -> Image {
        
        let src = image::open(file).unwrap(); // Returns image::DynamicImage
        let (w,h) = src.dimensions();
        let mut pixels = Vec::new();
        for y in 0..h {
            for x in 0..w {
                let p = src.get_pixel(x, y);
                pixels.push(p.data[0]);
                pixels.push(p.data[1]);
                pixels.push(p.data[2]);
                pixels.push(p.data[3]);
            }
        }
        Image{ 
            width: w as i32,
            height: h as i32,
            pixels: pixels
        }
    }

    // TODO: sanity checks
    /// Get pixel in a given x and y location of an image.
    ///
    /// # Examples
    ///
    /// ```
    /// use raster::image::Image;
    ///
    /// let mut image = Image::blank(2, 2);
    ///
    /// for y in 0..image.height {
    ///     for x in 0..image.width {
    ///         image.set_pixel( x, y, &[0,0,0,255]);
    ///         let pixel = image.get_pixel(x, y);
    ///         println!("pixel in ({},{}) = {:?}", x, y, pixel);
    ///     }
    /// }
    /// ```
    pub fn get_pixel(&self, x: i32, y:i32) -> &[u8] {
        let rgba = 4;
        let sx = (y * &self.width) + x;
        let start = sx * rgba;
        let end = start + rgba;
        
        &self.pixels[start as usize..end as usize]
    }

    pub fn get_color(&self, x: i32, y:i32) -> Color {
        let rgba = 4;
        let sx = (y * &self.width) + x;
        let start = sx * rgba;
        let end = start + rgba;
        
        let slice = &self.pixels[start as usize..end as usize];
        Color {
            r: slice[0],
            g: slice[1],
            b: slice[2],
            a: slice[3],
        }
    }

    // TODO: sanity checks
    /// Set pixel in a given x and y location of an image.
    ///
    /// # Examples
    ///
    /// ```
    /// use raster::image::Image;
    ///
    /// let mut image = Image::blank(2, 2);
    ///
    /// for y in 0..image.height {
    ///     for x in 0..image.width {
    ///         image.set_pixel( x, y, &[0,0,0,255]);
    ///         let pixel = image.get_pixel(x, y);
    ///         println!("pixel in ({},{}) = {:?}", x, y, pixel);
    ///     }
    /// }
    /// ```
    pub fn set_pixel(&mut self, x: i32, y:i32, pixel: &[u8]) {
        let rgba = 4; // length
        let sx = (y * &self.width) + x;
        let start = sx * rgba;
        
        self.pixels[start as usize] = pixel[0];
        self.pixels[start as usize + 1] = pixel[1];
        self.pixels[start as usize + 2] = pixel[2];
        self.pixels[start as usize + 3] = pixel[3];
    }
}