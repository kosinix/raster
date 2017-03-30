//!  A module for 2D transformation.


// from rust
use std::cmp;

// from external crate


// from local crate
use error::RasterResult;
use Image;
use Color;
use interpolate::{resample, InterpolationMode};
use position::PositionMode;
use editor::crop;

/// An enum for the various modes that can be used for transforming.
#[derive(Debug)]
pub enum TransformMode {
    /// Transform on x axis.
    Horizontal,
    /// Transform on y axis.
    Vertical
}

/// Flip an image on its x or y axis.
///
/// # Examples
///
/// ### Flip X:
///
/// ```
/// use raster::{transform, TransformMode};
///
/// //...
///
/// let mut image = raster::open("tests/in/sample.png").unwrap();
/// transform::flip(&mut image, TransformMode::Horizontal).unwrap();
/// raster::save(&image, "tests/out/test_transform_flip_x.png").unwrap();
/// ```
///
/// ![](https://kosinix.github.io/raster/out/test_transform_flip_x.png)
///
/// ### Flip Y:
///
/// ```
/// use raster::{transform, TransformMode};
///
/// //...
///
/// let mut image = raster::open("tests/in/sample.png").unwrap();
/// transform::flip(&mut image, TransformMode::Vertical).unwrap();
/// raster::save(&image, "tests/out/test_transform_flip_y.png").unwrap();
/// ```
///
/// ![](https://kosinix.github.io/raster/out/test_transform_flip_y.png)
///
pub fn flip(mut src: &mut Image, mode: TransformMode ) -> RasterResult<()> {

    let w: i32 = src.width;
    let h: i32 = src.height;

    match mode {
        TransformMode::Horizontal => {
            for x in 0..w {
                let src_x = x;
                let dest_x = w - x - 1;
                if dest_x <= src_x {
                    break;
                }
                for y in 0..h {

                    let pixel_left = try!(src.get_pixel(src_x, y));
                    let pixel_right = try!(src.get_pixel(dest_x, y));

                    try!(src.set_pixel(dest_x, y, &pixel_left));
                    try!(src.set_pixel(src_x, y, &pixel_right));

                }
            }

            Ok(())
        },
        TransformMode::Vertical => {
            for y in 0..h {
                let src_y = y;
                let dest_y = h - y - 1;
                if dest_y <= src_y {
                    break;
                }
                for x in 0..w {

                    let pixel_top = try!(src.get_pixel(x, src_y));
                    let pixel_bottom = try!(src.get_pixel(x, dest_y));

                    try!(src.set_pixel(x, dest_y, &pixel_top));
                    try!(src.set_pixel(x, src_y, &pixel_bottom));

                }
            }

            Ok(())
        }
    }

}

/// Rotate an image clockwise. Negate the degrees to do a counter-clockwise rotation. Background color can be any color.
///
/// Note: If you look closely, the quality for arbitrary angles is not very good due to the simple sampling algorithm. The 90, 180, and 270 angles looks fine because no pixels are lost. This will be fixed in the future with a better sampling algorithm.
///
/// # Examples
///
/// ### Rotate 45 degrees with a black background color:
///
/// ```
/// use raster::{transform, Color};
///
/// //...
///
/// let mut image = raster::open("tests/in/sample.png").unwrap();
/// transform::rotate(&mut image, 45, Color::rgb(0,0,0)).unwrap();
/// raster::save(&image, "tests/out/test_transform_rotate_45.png").unwrap();
/// ```
///
/// ![](https://kosinix.github.io/raster/out/test_transform_rotate_45.png)
///
///
/// ### Rotate 45 degrees counter-clockwise with a red background color:
///
/// ```
/// use raster::{transform, Color};
///
/// //...
///
/// let mut image = raster::open("tests/in/sample.png").unwrap();
/// transform::rotate(&mut image, -45, Color::rgb(252,145,145)).unwrap();
/// raster::save(&image, "tests/out/test_transform_rotate_45cc.png").unwrap();
/// ```
///
/// ![](https://kosinix.github.io/raster/out/test_transform_rotate_45cc.png)
///
pub fn rotate(mut src: &mut Image, degree: i32, bg: Color) -> RasterResult<()>{

    let w1 = src.width;
    let h1 = src.height;

    let degree = degree as f32; // convert to float

    // Using screen coords system, top left is always at (0,0)
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let top_right_1: (i32, i32) = (w1, 0);
    let top_right_2: (i32, i32) = _rotate(top_right_1, degree);
    min_x = cmp::min(min_x, top_right_2.0);
    max_x = cmp::max(max_x, top_right_2.0);
    min_y = cmp::min(min_y, top_right_2.1);
    max_y = cmp::max(max_y, top_right_2.1);

    let bottom_right_1: (i32, i32) = (w1, h1);
    let bottom_right_2: (i32, i32) = _rotate(bottom_right_1, degree);
    min_x = cmp::min(min_x, bottom_right_2.0);
    max_x = cmp::max(max_x, bottom_right_2.0);
    min_y = cmp::min(min_y, bottom_right_2.1);
    max_y = cmp::max(max_y, bottom_right_2.1);

    let bottom_left_1: (i32, i32) = (0, h1);
    let bottom_left_2: (i32, i32) = _rotate(bottom_left_1, degree);
    min_x = cmp::min(min_x, bottom_left_2.0);
    max_x = cmp::max(max_x, bottom_left_2.0);
    min_y = cmp::min(min_y, bottom_left_2.1);
    max_y = cmp::max(max_y, bottom_left_2.1);

    let w2 = ((min_x as f32).abs() + (max_x as f32).abs()) as i32 + 1;
    let h2 = ((min_y as f32).abs() + (max_y as f32).abs()) as i32 + 1;
    let mut dest = Image::blank(w2, h2);

    for (dest_y, y) in (0..).zip(min_y..max_y + 1) {
        for (dest_x, x) in (0..).zip(min_x..max_x + 1) {
            let point: (i32, i32) = _rotate((x,y), -degree);

            if point.0 >= 0 && point.0 < w1 && point.1 >=0 && point.1 < h1 {
                let pixel = try!(src.get_pixel(point.0, point.1));
                try!(dest.set_pixel(dest_x, dest_y, &pixel));
            } else {
                try!(dest.set_pixel(dest_x, dest_y, &Color::rgba(bg.r, bg.g, bg.b, bg.a)));
            }
        }
    }

    src.width = dest.width;
    src.height = dest.height;
    src.bytes = dest.bytes;

    Ok(())
}

/// Resize image to exact dimensions ignoring aspect ratio.
/// Useful if you want to force exact width and height.
pub fn resize_exact(mut src: &mut Image, w: i32, h: i32) -> RasterResult<()> {
    resample(src, w, h, InterpolationMode::Bicubic)
}

/// Resize image to exact height. Width is auto calculated.
/// Useful for creating row of images with the same height.
pub fn resize_exact_height(mut src: &mut Image, h: i32) -> RasterResult<()> {

    let width = src.width;
    let height = src.height;
    let ratio = width as f32 / height as f32;

    let resize_height = h;
    let resize_width = (h as f32 * ratio) as i32;

    resample(src, resize_width, resize_height, InterpolationMode::Bicubic)
}

/// Resize image to exact width. Height is auto calculated.
/// Useful for creating column of images with the same width.
pub fn resize_exact_width(mut src: &mut Image, w: i32) -> RasterResult<()> {
    let width  = src.width;
    let height = src.height;
    let ratio  = width as f32 / height as f32;

    let resize_width  = w;
    let resize_height = (w as f32 / ratio).round() as i32;

    resample(src, resize_width, resize_height, InterpolationMode::Bicubic)
}

/// Resize image to fill all the space in the given dimension. Excess parts are removed.
pub fn resize_fill(mut src: &mut Image, w: i32, h: i32) -> RasterResult<()> {
    let width  = src.width;
    let height = src.height;
    let ratio  = width as f32 / height as f32;

    // Base optimum size on new width
    let mut optimum_width  = w;
    let mut optimum_height = (w as f32 / ratio).round() as i32;

    if (optimum_width < w) || (optimum_height < h) { // Oops, where trying to fill and there are blank areas
        // So base optimum size on height instead
        optimum_width  = (h as f32 * ratio) as i32;
        optimum_height = h;
    }

    resample(src, optimum_width, optimum_height, InterpolationMode::Bicubic)
        .and_then(|_| crop(src, w, h, PositionMode::Center, 0, 0)) // Trim excess parts
}

/// Resize an image to fit within the given width and height.
/// The re-sized image will not exceed the given dimension.
/// Preserves the aspect ratio.
pub fn resize_fit(mut src: &mut Image, w: i32, h: i32) -> RasterResult<()> {

    let ratio: f64 = src.width as f64 / src.height as f64;

    // Try basing it on width first
    let mut resize_width  = w;
    let mut resize_height = (w as f64 / ratio).round() as i32;

    if (resize_width > w) || (resize_height > h) { // Oops, either width or height does not fit
        // So base on height instead
        resize_height = h;
        resize_width  = (h as f64 * ratio).round() as i32;
    }

    resample(src, resize_width, resize_height, InterpolationMode::Bicubic)
}

// Private functions

// Rotate a point clockwise to a given degree.
fn _rotate(p: (i32, i32), deg: f32) -> (i32, i32) {
    let radians:f32 = deg.to_radians();
    let px: f32 = p.0 as f32;
    let py: f32 = p.1 as f32;
    let cos = radians.cos();
    let sin = radians.sin();
    let x = ((px * cos) - (py * sin)).round();
    let y = ((px * sin) + (py * cos)).round();
    (x as i32, y as i32)
}
