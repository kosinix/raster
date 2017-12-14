//!  A module for handling colors.

// from rust
use std;

// from external crate

// from local crate
use error::{RasterError, RasterResult};

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
        if hex.len() == 9 && hex.starts_with('#') {
            // #FFFFFFFF (Red Green Blue Alpha)
            Ok(Color {
                r: _hex_dec(&hex[1..3])?,
                g: _hex_dec(&hex[3..5])?,
                b: _hex_dec(&hex[5..7])?,
                a: _hex_dec(&hex[7..9])?,
            })
        } else if hex.len() == 7 && hex.starts_with('#') {
            // #FFFFFF (Red Green Blue)
            Ok(Color {
                r: _hex_dec(&hex[1..3])?,
                g: _hex_dec(&hex[3..5])?,
                b: _hex_dec(&hex[5..7])?,
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
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
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
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
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
        let s = if v != 0.0 { chroma / v } else { 0.0 };

        (h.round() as u16, s * 100.0, v * 100.0)
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
    pub fn to_rgb(h: u16, s: f32, v: f32) -> (u8, u8, u8) {
        let h = h as f32 / 60.0;
        let s = s as f32 / 100.0; // Convert to 0.0 - 1.0
        let v = v as f32 / 100.0;

        let chroma = v * s;

        let x = chroma * (1.0 - ((h % 2.0) - 1.0).abs());

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
        (
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        )
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

fn rgb_min(r: f32, g: f32, b: f32) -> f32 {
    let min = if g < r { g } else { r };

    if b < min {
        b
    } else {
        min
    }
}

fn rgb_max(r: f32, g: f32, b: f32) -> f32 {
    let max = if g > r { g } else { r };

    if b > max {
        b
    } else {
        max
    }
}
