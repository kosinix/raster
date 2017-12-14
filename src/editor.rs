//!  A module for common image editing operations.

// from rust
use std::cmp;

// from external crate

// from local crate
use error::{RasterError, RasterResult};
use blend::{self, BlendMode};
use Color;
use Image;
use position::{Position, PositionMode};
use transform;

/// Blend 2 images into one. The image1 is the base and image2 is the top.
///
/// Opacity is any value from 0.0 - 1.0
///
/// The `offset_x` and `offset_y` are added to the final position. Can also be negative offsets.
///
/// # Errors
///
/// If image2 falls outside the canvas area, then this fails with
/// `RasterError::BlendingImageFallsOutsideCanvas`.
///
/// # Examples
/// ```
/// use raster::{editor, BlendMode, PositionMode};
///
/// // Create images from file
/// let image1 = raster::open("tests/in/sample.jpg").unwrap();
/// let image2 = raster::open("tests/in/watermark.png").unwrap();
///
/// // Blend image2 on top of image1 using normal mode, opacity of 1.0 (100%), with image2 at the
/// // center, with 0 x and 0 y offsets. whew
/// let normal = editor::blend(&image1, &image2, BlendMode::Normal, 1.0, PositionMode::Center, 0, 0).unwrap();
///
/// // All the other blend modes
/// let difference = editor::blend(&image1, &image2, BlendMode::Difference, 1.0, PositionMode::Center, 0, 0).unwrap();
/// let multiply = editor::blend(&image1, &image2, BlendMode::Multiply, 1.0, PositionMode::Center, 0, 0).unwrap();
/// let overlay = editor::blend(&image1, &image2, BlendMode::Overlay, 1.0, PositionMode::Center, 0, 0).unwrap();
/// let screen = editor::blend(&image1, &image2, BlendMode::Screen, 1.0, PositionMode::Center, 0, 0).unwrap();
///
/// // Save it
/// raster::save(&normal, "tests/out/test_blend_normal.png").unwrap();
/// raster::save(&difference, "tests/out/test_blend_difference.png").unwrap();
/// raster::save(&multiply, "tests/out/test_blend_multiply.png").unwrap();
/// raster::save(&overlay, "tests/out/test_blend_overlay.png").unwrap();
/// raster::save(&screen, "tests/out/test_blend_screen.png").unwrap();
/// ```
/// ### Source Images
///
/// Image 1
///
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
///
/// Image 2
///
/// ![](https://kosinix.github.io/raster/in/watermark.png)
///
/// ### Blended Images
///
/// Normal
///
/// ![](https://kosinix.github.io/raster/out/test_blend_normal.png)
///
/// Difference
///
/// ![](https://kosinix.github.io/raster/out/test_blend_difference.png)
///
///
/// Multiply
///
/// ![](https://kosinix.github.io/raster/out/test_blend_multiply.png)
///
///
/// Overlay
///
/// ![](https://kosinix.github.io/raster/out/test_blend_overlay.png)
///
///
/// Screen
///
/// ![](https://kosinix.github.io/raster/out/test_blend_screen.png)
///
pub fn blend(
    image1: &Image,
    image2: &Image,
    blend_mode: BlendMode,
    opacity: f32,
    position: PositionMode,
    offset_x: i32,
    offset_y: i32,
) -> RasterResult<Image> {
    let opacity = if opacity > 1.0 {
        1.0
    } else if opacity < 0.0 {
        0.0
    } else {
        opacity
    };

    // Turn into positioner struct
    let positioner = Position::new(position, offset_x, offset_y);

    // Position is for image2, image1 is canvas.
    let (offset_x, offset_y) =
        positioner.get_x_y(image1.width, image1.height, image2.width, image2.height)?;

    let (w1, h1) = (image1.width, image1.height);
    let (w2, h2) = (image2.width, image2.height);

    // Check if it overlaps
    if (offset_x >= w1) || (offset_x + w2 <= 0) || (offset_y >= h1) || (offset_y + h2 <= 0) {
        return Err(RasterError::BlendingImageFallsOutsideCanvas);
    }

    // Loop start X
    let mut loop_start_x = 0;
    let canvas_start_x = offset_x;
    if canvas_start_x < 0 {
        let diff = 0 - canvas_start_x;
        loop_start_x += diff;
    }

    // Loop end X
    let mut loop_end_x = w2;
    let canvas_end_x = offset_x + w2;
    if canvas_end_x > w1 {
        let diff = canvas_end_x - w1;
        loop_end_x -= diff;
    }

    // Loop start Y
    let mut loop_start_y = 0;
    let canvas_start_y = offset_y;
    if canvas_start_y < 0 {
        let diff = 0 - canvas_start_y;
        loop_start_y += diff;
    }

    // Loop end Y
    let mut loop_end_y = h2;
    let canvas_end_y = offset_y + h2;
    if canvas_end_y > h1 {
        let diff = canvas_end_y - h1;
        loop_end_y -= diff;
    }

    match blend_mode {
        BlendMode::Normal => blend::normal(
            image1,
            image2,
            loop_start_y,
            loop_end_y,
            loop_start_x,
            loop_end_x,
            offset_x,
            offset_y,
            opacity,
        ),
        BlendMode::Difference => blend::difference(
            image1,
            image2,
            loop_start_y,
            loop_end_y,
            loop_start_x,
            loop_end_x,
            offset_x,
            offset_y,
            opacity,
        ),
        BlendMode::Multiply => blend::multiply(
            image1,
            image2,
            loop_start_y,
            loop_end_y,
            loop_start_x,
            loop_end_x,
            offset_x,
            offset_y,
            opacity,
        ),
        BlendMode::Overlay => blend::overlay(
            image1,
            image2,
            loop_start_y,
            loop_end_y,
            loop_start_x,
            loop_end_x,
            offset_x,
            offset_y,
            opacity,
        ),
        BlendMode::Screen => blend::screen(
            image1,
            image2,
            loop_start_y,
            loop_end_y,
            loop_start_x,
            loop_end_x,
            offset_x,
            offset_y,
            opacity,
        ),
    }
}

/// Crop the image to the given dimension and position.
///
/// The `offset_x` and `offset_y` are added to the final position. Can also be negative offsets.
/// Offsets can be used to nudge the final position. Or you can set the position to
/// `PositionMode::TopLeft` and use the offsets as a normal screen x and y coordinates.
///
/// # Examples
///
/// ### Input
///
/// ![](https://kosinix.github.io/raster/in/crop-test.jpg)
///
/// ### Code
///
/// ```
/// use raster::{editor, PositionMode};
///
/// // Create image from file
/// let mut top_left = raster::open("tests/in/crop-test.jpg").unwrap();
///
/// // Make copies
/// let mut top_center = top_left.clone();
/// let mut top_right = top_left.clone();
///
/// let mut center_left = top_left.clone();
/// let mut center = top_left.clone();
/// let mut center_right = top_left.clone();
///
/// let mut bottom_left = top_left.clone();
/// let mut bottom_center = top_left.clone();
/// let mut bottom_right = top_left.clone();
///
/// // Crop it
/// editor::crop(&mut top_left, 167, 93, PositionMode::TopLeft, 0, 0).unwrap();
/// editor::crop(&mut top_center, 166, 93, PositionMode::TopCenter, 0, 0).unwrap();
/// editor::crop(&mut top_right, 167, 93, PositionMode::TopRight, 0, 0).unwrap();
///
/// editor::crop(&mut center_left, 167, 93, PositionMode::CenterLeft, 0, 0).unwrap();
/// editor::crop(&mut center, 166, 93, PositionMode::Center, 0, 0).unwrap();
/// editor::crop(&mut center_right, 167, 93, PositionMode::CenterRight, 0, 0).unwrap();
///
/// editor::crop(&mut bottom_left, 167, 93, PositionMode::BottomLeft, 0, 0).unwrap();
/// editor::crop(&mut bottom_center, 166, 93, PositionMode::BottomCenter, 0, 0).unwrap();
/// editor::crop(&mut bottom_right, 167, 93, PositionMode::BottomRight, 0, 0).unwrap();
///
/// // Save it
/// raster::save(&top_left, "tests/out/test_crop_top_left.jpg").unwrap();
/// raster::save(&top_center, "tests/out/test_crop_top_center.jpg").unwrap();
/// raster::save(&top_right, "tests/out/test_crop_top_right.jpg").unwrap();
///
/// raster::save(&center_left, "tests/out/test_crop_center_left.jpg").unwrap();
/// raster::save(&center, "tests/out/test_crop_center.jpg").unwrap();
/// raster::save(&center_right, "tests/out/test_crop_center_right.jpg").unwrap();
///
/// raster::save(&bottom_left, "tests/out/test_crop_bottom_left.jpg").unwrap();
/// raster::save(&bottom_center, "tests/out/test_crop_bottom_center.jpg").unwrap();
/// raster::save(&bottom_right, "tests/out/test_crop_bottom_right.jpg").unwrap();
/// ```
///
/// ### Output
/// The cropped images arranged in a grid, showing how you can easily set the crop position.
///
/// ![](https://kosinix.github.io/raster/out/test_crop_top_left.jpg) ![](https://kosinix.github.io/raster/out/test_crop_top_center.jpg) ![](https://kosinix.github.io/raster/out/test_crop_top_right.jpg)
/// ![](https://kosinix.github.io/raster/out/test_crop_center_left.jpg) ![](https://kosinix.github.io/raster/out/test_crop_center.jpg) ![](https://kosinix.github.io/raster/out/test_crop_center_right.jpg)
/// ![](https://kosinix.github.io/raster/out/test_crop_bottom_left.jpg) ![](https://kosinix.github.io/raster/out/test_crop_bottom_center.jpg) ![](https://kosinix.github.io/raster/out/test_crop_bottom_right.jpg)
pub fn crop(
    src: &mut Image,
    crop_width: i32,
    crop_height: i32,
    position: PositionMode,
    offset_x: i32,
    offset_y: i32,
) -> RasterResult<()> {
    // Turn into positioner struct
    let positioner = Position::new(position, offset_x, offset_y);

    let (offset_x, offset_y) = positioner.get_x_y(src.width, src.height, crop_width, crop_height)?;
    let offset_x = cmp::max(0, offset_x);
    let offset_y = cmp::max(0, offset_y);

    let height2 = {
        let height2 = offset_y + crop_height;
        cmp::min(height2, src.height)
    };

    let width2 = {
        let width2 = offset_x + crop_width;
        cmp::min(width2, src.width)
    };

    let mut dest = Image::blank(width2 - offset_x, height2 - offset_y);

    for y in 0..dest.height {
        for x in 0..dest.width {
            let pixel = src.get_pixel(offset_x + x, offset_y + y)?;
            dest.set_pixel(x, y, &Color::rgba(pixel.r, pixel.g, pixel.b, pixel.a))?;
        }
    }
    src.width = dest.width;
    src.height = dest.height;
    src.bytes = dest.bytes;

    Ok(())
}

/// Fill an image with color.
///
/// # Examples
/// ```
/// use raster::Image;
/// use raster::editor;
/// use raster::Color;
///
/// // Create a 100x100 image
/// let mut image = Image::blank(100, 100);
///
/// // Fill it with red
/// editor::fill(&mut image, Color::red()).unwrap();
///
/// // Save it
/// raster::save(&image, "tests/out/test_fill.png").unwrap();
/// ```
///
///
pub fn fill(src: &mut Image, color: Color) -> RasterResult<()> {
    for y in 0..src.height {
        for x in 0..src.width {
            src.set_pixel(x, y, &color)?;
        }
    }

    Ok(())
}

/// An enum for the various modes that can be used for resizing.
#[derive(Debug)]
pub enum ResizeMode {
    /// Resize image to exact dimensions ignoring aspect ratio.
    Exact,
    /// Resize image to exact width. Height parameter is ignored and is auto calculated instead.
    ExactWidth,
    /// Resize image to exact height. Width parameter is ignored and is auto calculated instead.
    ExactHeight,
    /// Resize an image to fit within the given width and height.
    Fit,
    /// Resize image to fill all the space in the given dimension. Excess parts are cropped.
    Fill,
}

/// Resize an image to a given width, height and mode.
///
/// # Examples
/// ### Resize Fit
/// ```
/// use raster::{editor, Color, Image, ResizeMode, BlendMode, PositionMode};
///
/// // Create an image from file
/// let mut image1 = raster::open("tests/in/sample.jpg").unwrap();
/// let mut image2 = raster::open("tests/in/portrait.jpg").unwrap();
///
/// // Resize it
/// editor::resize(&mut image1, 200, 200, ResizeMode::Fit).unwrap();
/// editor::resize(&mut image2, 200, 200, ResizeMode::Fit).unwrap();
///
/// // Superimpose images on a gray background
/// let mut bg = Image::blank(200, 200);
/// editor::fill(&mut bg, Color::hex("#CCCCCC").unwrap()).unwrap();
///
/// let image1 = editor::blend(&bg, &image1, BlendMode::Normal, 1.0, PositionMode::TopLeft, 0, 0).unwrap();
/// let image2 = editor::blend(&bg, &image2, BlendMode::Normal, 1.0, PositionMode::TopLeft, 0, 0).unwrap();
///
/// raster::save(&image1, "tests/out/test_resize_fit_1.jpg").unwrap();
/// raster::save(&image2, "tests/out/test_resize_fit_2.jpg").unwrap();
/// ```
///
/// The gray box shows the 200x200 imaginary box that the images "fit" in.
///
///
/// ![](https://kosinix.github.io/raster/out/test_resize_fit_1.jpg) ![](https://kosinix.github.io/raster/out/test_resize_fit_2.jpg)
///
/// ### Resize Fill
/// ```
/// use raster::{editor, Color, Image, ResizeMode};
///
/// // Create an image from file
/// let mut image1 = raster::open("tests/in/sample.jpg").unwrap();
/// let mut image2 = raster::open("tests/in/portrait.jpg").unwrap();
///
/// // Resize it
/// editor::resize(&mut image1, 200, 200, ResizeMode::Fill).unwrap();
/// editor::resize(&mut image2, 200, 200, ResizeMode::Fill).unwrap();
///
/// raster::save(&image1, "tests/out/test_resize_fill_1.jpg").unwrap();
/// raster::save(&image2, "tests/out/test_resize_fill_2.jpg").unwrap();
/// ```
///
/// The image fills up the entire 200x200 box.
///
/// ![](https://kosinix.github.io/raster/out/test_resize_fill_1.jpg) ![](https://kosinix.github.io/raster/out/test_resize_fill_2.jpg)
///
/// ### Resize to Exact Width
/// ```
/// use raster::{editor, Color, Image, ResizeMode};
///
/// // Create an image from file
/// let mut image1 = raster::open("tests/in/sample.jpg").unwrap();
/// let mut image2 = raster::open("tests/in/portrait.jpg").unwrap();
///
/// // Resize it
/// editor::resize(&mut image1, 200, 200, ResizeMode::ExactWidth).unwrap();
/// editor::resize(&mut image2, 200, 200, ResizeMode::ExactWidth).unwrap();
///
/// raster::save(&image1, "tests/out/test_resize_exact_width_1.jpg").unwrap();
/// raster::save(&image2, "tests/out/test_resize_exact_width_2.jpg").unwrap();
/// ```
///
/// The images will have a width of 200. The height is auto-calculated.
///
/// ![](https://kosinix.github.io/raster/out/test_resize_exact_width_1.jpg)
/// ![](https://kosinix.github.io/raster/out/test_resize_exact_width_2.jpg)
///
/// ### Resize to Exact Height
/// ```
/// use raster::{editor, Color, Image, ResizeMode};
///
/// // Create an image from file
/// let mut image1 = raster::open("tests/in/sample.jpg").unwrap();
/// let mut image2 = raster::open("tests/in/portrait.jpg").unwrap();
///
/// // Resize it
/// editor::resize(&mut image1, 200, 200, ResizeMode::ExactHeight).unwrap();
/// editor::resize(&mut image2, 200, 200, ResizeMode::ExactHeight).unwrap();
///
/// raster::save(&image1, "tests/out/test_resize_exact_height_1.jpg").unwrap();
/// raster::save(&image2, "tests/out/test_resize_exact_height_2.jpg").unwrap();
/// ```
///
/// The images will have a height of 200. The width is auto-calculated.
///
/// ![](https://kosinix.github.io/raster/out/test_resize_exact_height_1.jpg) ![](https://kosinix.github.io/raster/out/test_resize_exact_height_2.jpg)
///
/// ### Resize to Exact Dimension
/// ```
/// use raster::{editor, Color, Image, ResizeMode};
///
/// // Create an image from file
/// let mut image1 = raster::open("tests/in/sample.jpg").unwrap();
/// let mut image2 = raster::open("tests/in/portrait.jpg").unwrap();
///
/// // Resize it
/// editor::resize(&mut image1, 200, 200, ResizeMode::Exact).unwrap();
/// editor::resize(&mut image2, 200, 200, ResizeMode::Exact).unwrap();
///
/// raster::save(&image1, "tests/out/test_resize_exact_1.jpg").unwrap();
/// raster::save(&image2, "tests/out/test_resize_exact_2.jpg").unwrap();
/// ```
///
/// The images will be resized to the exact dimension ignoring aspect ratio.
///
/// ![](https://kosinix.github.io/raster/out/test_resize_exact_1.jpg) ![](https://kosinix.github.io/raster/out/test_resize_exact_2.jpg)
///
pub fn resize(src: &mut Image, w: i32, h: i32, mode: ResizeMode) -> RasterResult<()> {
    match mode {
        ResizeMode::Exact => transform::resize_exact(src, w, h),
        ResizeMode::ExactWidth => transform::resize_exact_width(src, w),
        ResizeMode::ExactHeight => transform::resize_exact_height(src, h),
        ResizeMode::Fit => transform::resize_fit(src, w, h),
        ResizeMode::Fill => transform::resize_fill(src, w, h),
    }
}
