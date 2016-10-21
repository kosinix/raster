//!  A module for handling colors.

// crates

// from rust

// from external crate

// from local crate

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
    /// use raster::color::Color;
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
    /// use raster::color::Color;
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

    pub fn clone(&self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}
