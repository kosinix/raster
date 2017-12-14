//!  A module for generic representation of image.

// from rust
use std::collections::HashMap;

// from external crate

// from local crate
use error::{RasterError, RasterResult};
use color::Color;

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
    pub fn blank(w: i32, h: i32) -> Image {
        let mut bytes = Vec::with_capacity((w * h) as usize * 4);
        for _ in 0..h {
            for _ in 0..w {
                bytes.extend_from_slice(&[0, 0, 0, 255]);
            }
        }
        Image {
            width: w,
            height: h,
            bytes: bytes,
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
    pub fn check_pixel(&self, x: i32, y: i32) -> bool {
        if y < 0 || y > self.height {
            // TODO: check on actual vectors and not just width and height?
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
    ///             let height = (canvas_h as f32 * (*count as f32 / max_r_bin as f32)).round() as i32;
    ///
    ///             for y in canvas_h-height..canvas_h {
    ///                 image.set_pixel(x, y, &Color::hex("#e22d11").unwrap()).unwrap();
    ///             }
    ///         },
    ///         None => {}
    ///     }
    /// }
    ///
    /// raster::save(&image, "tests/out/histogram.png").unwrap();
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
                let pixel = self.get_pixel(x, y)?;

                // Insert the key with a value of 0 if key does not exist yet. Then return the
                // count (which is zero).
                let r_bin_c = r_bin.entry(pixel.r).or_insert(0);
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
    pub fn get_pixel(&self, x: i32, y: i32) -> RasterResult<Color> {
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
    /// let _ = image.set_pixel(0, 0, &Color::rgba(255, 0, 0, 255)); // Set first pixel to red
    ///
    /// let pixel = image.get_pixel(0, 0).unwrap();
    ///
    /// assert_eq!(255, pixel.r);
    /// assert_eq!(0, pixel.g);
    /// assert_eq!(0, pixel.b);
    /// assert_eq!(255, pixel.a);
    /// ```
    pub fn set_pixel(&mut self, x: i32, y: i32, color: &Color) -> RasterResult<()> {
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

/// Holds histogram information.
pub type Histogram = (
    HashMap<u8, u32>,
    HashMap<u8, u32>,
    HashMap<u8, u32>,
    HashMap<u8, u32>,
);

/// Enumeration of supported raster formats.
#[derive(Debug)]
pub enum ImageFormat {
    Gif,
    Jpeg,
    Png,
}
