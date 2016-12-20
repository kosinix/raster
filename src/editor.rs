//!  A module for common image editing operations.

// crates
extern crate image;

// from rust

// from external crate


// from local crate
use blend;
use Color;
use Image;
use position::Position;
use transform;

/// Blend 2 images into one. The image1 is the base and image2 is the top. 
/// 
/// Supported blend modes: 
///
/// * normal
/// * difference
/// * multiply
/// * overlay
/// * screen
///
/// Possible position: 
///
/// * top-left
/// * top-center
/// * top-right
/// * center-left
/// * center
/// * center-right
/// * bottom-left
/// * bottom-center
/// * bottom-right
///
/// Opacity is any value from 0.0 - 1.0
///
/// The offset_x and offset_y are added to the final position. Can also be negative offsets.
///
/// # Examples
/// ```
/// use raster::editor;
///
/// // Create images from file
/// let image1 = raster::open("tests/in/sample.jpg").unwrap();
/// let image2 = raster::open("tests/in/watermark.png").unwrap();
/// 
/// // Blend image2 on top of image1 using normal mode, opacity of 1.0 (100%), with image2 at the center, with 0 x and 0 y offsets. whew
/// let normal = editor::blend(&image1, &image2, "normal", 1.0, "center", 0, 0).unwrap();
///
/// // All the other blend modes
/// let difference = editor::blend(&image1, &image2, "difference", 1.0, "center", 0, 0).unwrap();
/// let multiply = editor::blend(&image1, &image2, "multiply", 1.0, "center", 0, 0).unwrap();
/// let overlay = editor::blend(&image1, &image2, "overlay", 1.0, "center", 0, 0).unwrap();
/// let screen = editor::blend(&image1, &image2, "screen", 1.0, "center", 0, 0).unwrap();
///
/// // Save it
/// raster::save(&normal, "tests/out/test_blend_normal.png");
/// raster::save(&difference, "tests/out/test_blend_difference.png");
/// raster::save(&multiply, "tests/out/test_blend_multiply.png");
/// raster::save(&overlay, "tests/out/test_blend_overlay.png");
/// raster::save(&screen, "tests/out/test_blend_screen.png");
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
pub fn blend<'a>(image1: &Image, image2: &Image, blend_mode: &str, opacity: f32, position: &str, offset_x: i32, offset_y: i32) -> Result<Image, String> {
    
    let mut opacity = opacity;
    if opacity > 1.0 {
        opacity = 1.0
    } else if opacity < 0.0 {
        opacity = 0.0
    }

    // Turn into positioner struct
    let positioner = Position::new(position, offset_x, offset_y);

    // Position is for image2, image1 is canvas.
    let (offset_x, offset_y) = try!(positioner.get_x_y( image1.width, image1.height, image2.width, image2.height));

    let (w1, h1) = (image1.width, image1.height);
    let (w2, h2) = (image2.width, image2.height);

    // Check if it overlaps
    if (offset_x >= w1 ) ||
        (offset_x + w2 <= 0) ||
        (offset_y >= h1) ||
        (offset_y + h2 <= 0) {

        return Err("Invalid blending. Image 2 is outside the canvas.".to_string());
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
    if canvas_end_x > w1{
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

    let blend_mode = blend_mode.to_lowercase();
    match &*blend_mode {
        "normal" => {
            let image3 = try!(blend::normal( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity ));
            Ok(image3)
        },
        "difference" => {
            let image3 = try!(blend::difference( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity ));
            Ok(image3)
        },
        "multiply" => {
            let image3 = try!(blend::multiply( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity ));
            Ok(image3)
        },
        "overlay" => {
            let image3 = try!(blend::overlay( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity ));
            Ok(image3)
        },
        "screen" => {
            let image3 = try!(blend::screen( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity ));
            Ok(image3)
        },
        _ => {
            Err(format!("Invalid blend type {}.", &*blend_mode))
        }
    }
}

/// Crop the image to the given dimension and position.
///
/// Possible position: 
///
/// * top-left
/// * top-center
/// * top-right
/// * center-left
/// * center
/// * center-right
/// * bottom-left
/// * bottom-center
/// * bottom-right
///
/// The offset_x and offset_y are added to the final position. Can also be negative offsets. Offsets can be used to nudge the final position. Or you can set the position to "top-left" and use the offsets as a normal screen x and y coordinates.
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
/// use raster::editor;
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
/// editor::crop(&mut top_left, 167, 93, "top-left", 0, 0).unwrap();
/// editor::crop(&mut top_center, 166, 93, "top-center", 0, 0).unwrap();
/// editor::crop(&mut top_right, 167, 93, "top-right", 0, 0).unwrap();
///
/// editor::crop(&mut center_left, 167, 93, "center-left", 0, 0).unwrap();
/// editor::crop(&mut center, 166, 93, "center", 0, 0).unwrap();
/// editor::crop(&mut center_right, 167, 93, "center-right", 0, 0).unwrap();
///
/// editor::crop(&mut bottom_left, 167, 93, "bottom-left", 0, 0).unwrap();
/// editor::crop(&mut bottom_center, 166, 93, "bottom-center", 0, 0).unwrap();
/// editor::crop(&mut bottom_right, 167, 93, "bottom-right", 0, 0).unwrap();
///
/// // Save it
/// raster::save(&top_left, "tests/out/test_crop_top_left.jpg");
/// raster::save(&top_center, "tests/out/test_crop_top_center.jpg");
/// raster::save(&top_right, "tests/out/test_crop_top_right.jpg");
///
/// raster::save(&center_left, "tests/out/test_crop_center_left.jpg");
/// raster::save(&center, "tests/out/test_crop_center.jpg");
/// raster::save(&center_right, "tests/out/test_crop_center_right.jpg");
///
/// raster::save(&bottom_left, "tests/out/test_crop_bottom_left.jpg");
/// raster::save(&bottom_center, "tests/out/test_crop_bottom_center.jpg");
/// raster::save(&bottom_right, "tests/out/test_crop_bottom_right.jpg");
/// ```
///
/// ### Output
/// The cropped images arranged in a grid, showing how you can easily set the crop position.
///
/// ![](https://kosinix.github.io/raster/out/test_crop_top_left.jpg)
/// ![](https://kosinix.github.io/raster/out/test_crop_top_center.jpg)
/// ![](https://kosinix.github.io/raster/out/test_crop_top_right.jpg)  
/// ![](https://kosinix.github.io/raster/out/test_crop_center_left.jpg)
/// ![](https://kosinix.github.io/raster/out/test_crop_center.jpg)
/// ![](https://kosinix.github.io/raster/out/test_crop_center_right.jpg)  
/// ![](https://kosinix.github.io/raster/out/test_crop_bottom_left.jpg)
/// ![](https://kosinix.github.io/raster/out/test_crop_bottom_center.jpg)
/// ![](https://kosinix.github.io/raster/out/test_crop_bottom_right.jpg)
///
pub fn crop<'a>(mut src: &'a mut Image, crop_width: i32, crop_height: i32, position: &str, offset_x: i32, offset_y: i32) -> Result<(), String> {

    // Turn into positioner struct
    let positioner = Position::new(position, offset_x, offset_y);

    let (offset_x, offset_y) = try!(positioner.get_x_y( src.width, src.height, crop_width, crop_height));
    let offset_x = if offset_x < 0 { 0 } else { offset_x };
    let offset_y = if offset_y < 0 { 0 } else { offset_y };


    let mut height2 = offset_y + crop_height;
    if height2 > src.height { 
        height2 = src.height 
    }

    let mut width2 = offset_x + crop_width;
    if width2 > src.width { 
        width2 = src.width 
    }

    let mut dest = Image::blank(width2-offset_x, height2-offset_y);

    for y in 0..dest.height {
        for x in 0..dest.width {
            let pixel = try!(src.get_pixel(offset_x + x, offset_y + y));
            try!(dest.set_pixel(x, y, Color::rgba(pixel.r, pixel.g, pixel.b, pixel.a)));
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
/// raster::save(&image, "tests/out/test_fill.png");
/// ```
/// 
///
pub fn fill(mut src: &mut Image, color: Color) -> Result<(), String> {

    for y in 0..src.height {
        for x in 0..src.width {
            try!(src.set_pixel(x, y, color.clone()));
        }
    }

    Ok(())
}

/// Resize an image to a given width, height and mode.
///
/// Modes:
///
/// * exact - Resize image to exact dimensions ignoring aspect ratio. 
/// * exact_width - Resize image to exact width. Height parameter is ignored and is auto calculated instead.
/// * exact_height - Resize image to exact height. Width parameter is ignored and is auto calculated instead.
/// * fit - Resize an image to fit within the given width and height.
/// * fill - Resize image to fill all the space in the given dimension. Excess parts are cropped.
///
/// # Examples
/// ### fit
/// ```
/// use raster::editor;
///
/// // Create an image from file
/// let mut image = raster::open("tests/in/sample.png").unwrap();
/// 
/// editor::resize(&mut image, 200, 200, "fit").unwrap();
/// raster::save(&image, "tests/out/test_resize_fit.png");
/// ```
///
/// ![](https://kosinix.github.io/raster/out/test_resize_fit.png)
///
/// ### fill
/// ```
/// use raster::editor;
///
/// // Create an image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// 
/// editor::resize(&mut image, 200, 200, "fill").unwrap();
/// raster::save(&image, "tests/out/test_resize_fill.jpg");
/// ```
///
/// ![](https://kosinix.github.io/raster/out/test_resize_fill.jpg)
///
/// ### exact width
/// ```
/// use raster::editor;
///
/// // Create an image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// 
/// editor::resize(&mut image, 200, 0, "exact_width").unwrap();
/// raster::save(&image, "tests/out/test_resize_exact_width.jpg");
/// ```
///
/// ![](https://kosinix.github.io/raster/out/test_resize_exact_width.jpg)
///
/// ### exact height
/// ```
/// use raster::editor;
///
/// // Create an image from file
/// let mut image = raster::open("tests/in/portrait.jpg").unwrap();
/// 
/// editor::resize(&mut image, 0, 200, "exact_height").unwrap();
/// raster::save(&image, "tests/out/test_resize_exact_height.jpg");
/// ```
///
/// ![](https://kosinix.github.io/raster/out/test_resize_exact_width.jpg)
///
pub fn resize<'a>(mut src: &'a mut Image, w: i32, h: i32, mode: &str) -> Result<(), String> {
    
    match mode {
        "exact" => {
            try!(transform::resize_exact(&mut src, w, h));
            Ok(())
        }
        "exact_width" => {
            try!(transform::resize_exact_width(&mut src, w));
            Ok(())
        }
        "exact_height" => {
            try!(transform::resize_exact_height(&mut src, h));
            Ok(())
        }
        "fit" => {
            try!(transform::resize_fit(&mut src, w, h));
            Ok(())
        },
        "fill" => {
            try!(transform::resize_fill(&mut src, w, h));
            Ok(())
        },
        _ => {
            Err(format!("Invalid resize mode '{}'.", mode))
        },
    }
}
