//!  A module for filtering pixels.

// from rust
use std::cmp;

// from external crate

// from local crate
use error::{RasterError, RasterResult};
use Image;
use Color;

/// An enum for the various modes that can be used for blurring.
#[derive(Debug)]
pub enum BlurMode {
    Box,
    Gaussian,
}

/// An enum to specify orientation of a filter.
#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
    DiagonalUp,
    DiagonalDown,
    DiagonalBoth,
    Both,
}

/// Apply box or Gaussian blur.
///
/// # Examples
/// ### Box Blur
///
/// ```
/// use raster::{filter, BlurMode};
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::blur(&mut image, BlurMode::Box).unwrap();
/// raster::save(&image, "tests/out/test_filter_box_blur.jpg").unwrap();
/// ```
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
///
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_box_blur.jpg)
///
/// ### Gaussian Blur
///
/// ```
/// use raster::{filter, BlurMode};
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::blur(&mut image, BlurMode::Gaussian).unwrap();
/// raster::save(&image, "tests/out/test_filter_gaussian_blur.jpg").unwrap();
/// ```
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
///
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_gaussian_blur.jpg)
///
pub fn blur(src: &mut Image, mode: BlurMode) -> RasterResult<()> {
    match mode {
        BlurMode::Box => blur_box(src),
        BlurMode::Gaussian => blur_gaussian(src),
    }
}

/// Apply brightness.
///
/// A brightness of < 0.0 will darken the image and brightness of > 1.0 will lighten it.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::brightness(&mut image, 1.5).unwrap();
/// raster::save(&image, "tests/out/test_filter_brightness.jpg").unwrap();
/// ```
///
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
///
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_brightness.jpg)
///
pub fn brightness(src: &mut Image, factor: f32) -> RasterResult<()> {
    let w: i32 = src.width;
    let h: i32 = src.height;

    // if gamma < 0.01 || gamma > 9.99{
    //     return Err(format!("Incorrect gamma value {}. Must be in range 0.01 - 9.99.", gamma));
    // }
    // let factor = 255.0 * factor;

    for y in 0..h {
        for x in 0..w {
            let p = src.get_pixel(x, y)?;
            let r = cmp::max(0, cmp::min(255, (p.r as f32 * factor) as i32));
            let g = cmp::max(0, cmp::min(255, (p.g as f32 * factor) as i32));
            let b = cmp::max(0, cmp::min(255, (p.b as f32 * factor) as i32));
            // TODO: Should alpha be included?
            let a = cmp::max(0, cmp::min(255, (p.a as f32 * factor) as i32));

            src.set_pixel(x, y, &Color::rgba(r as u8, g as u8, b as u8, a as u8))?;
        }
    }

    Ok(())
}

/// Apply a convolution matrix.
///
/// The divisor is applied as the last step of convolution.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// let matrix: [[i32; 3]; 3] = [
///     [0, 0, 0],
///     [0, 1, 0],
///     [0, 0, 0]
/// ];
/// filter::convolve(&mut image, matrix, 1).unwrap();
/// raster::save(&image, "tests/out/test_filter_convolve.jpg").unwrap();
/// ```
pub fn convolve(src: &mut Image, matrix: [[i32; 3]; 3], divisor: i32) -> RasterResult<()> {
    let w: i32 = src.width;
    let h: i32 = src.height;
    let m_size = 3; // Matrix size

    let copy = src.clone(); // Create a copy as input of pixels

    for y in 0..h {
        for x in 0..w {
            let mstarty = y - 1;
            let mstartx = x - 1;

            let mut accum_red: i32 = 0;
            let mut accum_green: i32 = 0;
            let mut accum_blue: i32 = 0;
            let mut accum_alpha: i32 = 0;

            for (m_index_y, mut src_y) in (0..).zip(mstarty..mstarty + m_size) {
                if src_y < 0 {
                    src_y = 0;
                } else if src_y > h - 1 {
                    src_y = h - 1;
                }

                for (m_index_x, mut src_x) in (0..).zip(mstartx..mstartx + m_size) {
                    if src_x < 0 {
                        src_x = 0;
                    } else if src_x > w - 1 {
                        src_x = w - 1;
                    }

                    let pixel = copy.get_pixel(src_x, src_y)?;
                    accum_red += pixel.r as i32 * matrix[m_index_y][m_index_x];
                    accum_green += pixel.g as i32 * matrix[m_index_y][m_index_x];
                    accum_blue += pixel.b as i32 * matrix[m_index_y][m_index_x];
                    accum_alpha += pixel.a as i32 * matrix[m_index_y][m_index_x];
                }
            }

            if divisor != 1 {
                accum_red /= divisor;
                accum_green /= divisor;
                accum_blue /= divisor;
                accum_alpha /= divisor;
            }

            if accum_red < 0 {
                accum_red = 0;
            }
            if accum_green < 0 {
                accum_green = 0;
            }
            if accum_blue < 0 {
                accum_blue = 0;
            }
            if accum_alpha < 0 {
                accum_alpha = 0;
            }

            if accum_red > 255 {
                accum_red = 255;
            }
            if accum_green > 255 {
                accum_green = 255;
            }
            if accum_blue > 255 {
                accum_blue = 255;
            }
            if accum_alpha > 255 {
                accum_alpha = 255;
            }

            src.set_pixel(
                x,
                y,
                &Color::rgba(
                    accum_red as u8,
                    accum_green as u8,
                    accum_blue as u8,
                    accum_alpha as u8,
                ),
            )?;
        }
    }

    Ok(())
}

/// Apply emboss.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::emboss(&mut image).unwrap();
/// raster::save(&image, "tests/out/test_filter_emboss.jpg").unwrap();
/// ```
///
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
///
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_emboss.jpg)
///
pub fn emboss(src: &mut Image) -> RasterResult<()> {
    let matrix: [[i32; 3]; 3] = [[-2, -1, 0], [-1, 1, 1], [0, 1, 2]];
    convolve(src, matrix, 1)
}

/// Apply Sobel edge detection.
///
/// # Examples
/// ```
/// use raster::{filter, Orientation};
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::sobel(&mut image, Orientation::Horizontal).unwrap();
/// raster::save(&image, "tests/out/test_filter_sobel_x.jpg").unwrap();
/// ```
///
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
///
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_sobel_x.jpg)
///
pub fn sobel(src: &mut Image, mode: Orientation) -> RasterResult<()> {
    grayscale(src)?;
    let matrix = match mode {
        Orientation::Horizontal => [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]],
        Orientation::Vertical => [[-1, -2, -1], [0, 0, 0], [1, 2, 1]],
        Orientation::DiagonalUp => [[0, -1, -2], [1, 0, -1], [2, 1, 0]],
        Orientation::DiagonalDown => [[-2, -1, 0], [-1, 0, 1], [0, 1, 2]],
        Orientation::Both => {
            return sobel_both(
                src,
                [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]],
                [[-1, -2, -1], [0, 0, 0], [1, 2, 1]],
            )
        }
        Orientation::DiagonalBoth => {
            return sobel_both(
                src,
                [[0, -1, -2], [1, 0, -1], [2, 1, 0]],
                [[-2, -1, 0], [-1, 0, 1], [0, 1, 2]],
            )
        }
    };
    convolve(src, matrix, 1)
}

fn sobel_both(
    src: &mut Image,
    matrix_one: [[i32; 3]; 3],
    matrix_two: [[i32; 3]; 3],
) -> RasterResult<()> {
    let mut image_x = src.clone();
    let mut image_y = src.clone();
    convolve(&mut image_x, matrix_one, 1)?;
    convolve(&mut image_y, matrix_two, 1)?;

    let w: i32 = src.width;
    let h: i32 = src.height;
    for y in 0..h {
        for x in 0..w {
            let pixel_x = image_x.get_pixel(x, y)?;
            let pixel_y = image_y.get_pixel(x, y)?;
            // Calculate the sum of the derivatives with sqrt((dImage/dx)²+(dImage/dy)²)
            let pixel = ((pixel_x.r as f64).powi(2) + (pixel_y.r as f64).powi(2)).sqrt();
            src.set_pixel(
                x,
                y,
                &Color::rgba(pixel as u8, pixel as u8, pixel as u8, pixel_x.a as u8),
            )?;
        }
    }

    Ok(())
}

/// Apply a gamma correction.
///
/// Gamma can be a value from 0.01 - 9.99.
/// A gamma < 1.0 will darken and a gamma > 1.0 will lighten the image.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::gamma(&mut image, 2.0).unwrap();
/// raster::save(&image, "tests/out/test_filter_gamma.jpg").unwrap();
/// ```
///
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
///
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_gamma.jpg)
///
// http://stackoverflow.com/questions/14088889/changing-a-color-brightness
pub fn gamma(src: &mut Image, gamma: f32) -> RasterResult<()> {
    let w: i32 = src.width;
    let h: i32 = src.height;

    if gamma < 0.01 || gamma > 9.99 {
        return Err(RasterError::InvalidGamma(gamma));
    }

    for y in 0..h {
        for x in 0..w {
            let p = src.get_pixel(x, y)?;
            let r = (p.r as f32 / 255.0).powf(gamma) * 255.0;
            let g = (p.g as f32 / 255.0).powf(gamma) * 255.0;
            let b = (p.b as f32 / 255.0).powf(gamma) * 255.0;

            src.set_pixel(x, y, &Color::rgba(r as u8, g as u8, b as u8, p.a as u8))?;
        }
    }

    Ok(())
}

/// Turn into grayscale image.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::grayscale(&mut image).unwrap();
/// raster::save(&image, "tests/out/test_filter_grayscale.jpg").unwrap();
/// ```
///
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
///
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_grayscale.jpg)
///
pub fn grayscale(src: &mut Image) -> RasterResult<()> {
    let w: i32 = src.width;
    let h: i32 = src.height;

    for y in 0..h {
        for x in 0..w {
            let p = src.get_pixel(x, y)?;
            let gray = (p.r as f32 * 0.3) + (p.g as f32 * 0.59) + (p.b as f32 * 0.11);

            src.set_pixel(
                x,
                y,
                &Color::rgba(gray as u8, gray as u8, gray as u8, gray as u8),
            )?;
        }
    }

    Ok(())
}

/// Change saturation.
///
/// Pass a float value for sat. < 0.0 to decrease and > 0.0 to increase. Eg 0.5 for 50% increase
/// in saturation.
///
/// Note: Saturation does not look good at the moment.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.png").unwrap();
/// filter::saturation(&mut image, 0.5).unwrap();
/// raster::save(&image, "tests/out/test_filter_saturation.jpg").unwrap();
/// ```
///
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.png)
///
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_saturation.jpg)
///
pub fn saturation(src: &mut Image, sat: f32) -> RasterResult<()> {
    let w: i32 = src.width;
    let h: i32 = src.height;

    for y in 0..h {
        for x in 0..w {
            let p = src.get_pixel(x, y)?;
            let hsv = Color::to_hsv(p.r, p.g, p.b);
            let s = hsv.1;
            let factor = (100.0 - s) * sat; // use % remaining
            let mut new_s = s + factor;
            if new_s > 100.0 {
                new_s = 100.0;
            } else if new_s < 0.0 {
                new_s = 0.0;
            }
            let rgb = Color::to_rgb(hsv.0, new_s, hsv.2);

            src.set_pixel(x, y, &Color::rgb(rgb.0, rgb.1, rgb.2))?;
        }
    }

    Ok(())
}

/// Apply sharpen.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::sharpen(&mut image).unwrap();
/// raster::save(&image, "tests/out/test_filter_sharpen.jpg").unwrap();
/// ```
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
///
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_sharpen.jpg)
///
pub fn sharpen(src: &mut Image) -> RasterResult<()> {
    let matrix: [[i32; 3]; 3] = [[0, -1, 0], [-1, 5, -1], [0, -1, 0]];
    convolve(src, matrix, 1)
}

// Private functions

// Box
fn blur_box(src: &mut Image) -> RasterResult<()> {
    let matrix: [[i32; 3]; 3] = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    convolve(src, matrix, 9)
}

// Gaussian
fn blur_gaussian(src: &mut Image) -> RasterResult<()> {
    let matrix: [[i32; 3]; 3] = [[1, 2, 1], [2, 4, 2], [1, 2, 1]];
    convolve(src, matrix, 16)
}
