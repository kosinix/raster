//!  A module for editing images.

// crates
extern crate image;

// from rust
use std::path::Path;

// from external crate


// from local crate
use blend;
use image::Image;
use position::Position;

/// Blend 2 images into one. The image1 is the base and image2 is the top. 
/// 
/// Supported blend modes: "normal", "difference", multiply", "overlay", "screen"
/// Position: "top-left", "top-center", "top-right", "center-left", "center", "center-right", "bottom-left", "bottom-center", "bottom-right"
/// Opacity is any value from 0.0 - 1.0
/// offset_x and offset_y are added to the final position. Can also be negative offsets.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create images from file
/// let image1 = Image::from_file("tests/image/sample.jpg");
/// let image2 = Image::from_file("tests/image/watermark.png");
/// 
/// // Blend image2 on top of image1 using normal mode, opacity of 1.0 (100%), with image2 at the center, with 0 x and 0 y offsets. whew
/// let image3 = editor::blend(&image1, &image2, "normal", 1.0, "center", 0, 0);
///
/// // Save it
/// editor::save(&image3, "tests/out/test_blend_normal.png");
/// ```
pub fn blend<'a>(image1: &Image, image2: &Image, blend_mode: &str, opacity: f32, position: &str, offset_x: i32, offset_y: i32) -> Image {
    
    let mut opacity = opacity;
    if opacity > 1.0 {
        opacity = 1.0
    } else if opacity < 0.0 {
        opacity = 0.0
    }

    // Turn into positioner struct
    let positioner = Position::new(position, offset_x, offset_y);

    // Position is for image2, image1 is canvas.
    let (offset_x, offset_y) = positioner.get_x_y( image1.width, image1.height, image2.width, image2.height);

    let (w1, h1) = (image1.width, image1.height);
    let (w2, h2) = (image2.width, image2.height);

    // Check if it overlaps
    if (offset_x >= w1 ) ||
        (offset_x + w2 <= 0) ||
        (offset_y >= h1) ||
        (offset_y + h2 <= 0) {

        panic!("Invalid blending. Image 2 is outside the canvas."); // TODO: Proper error handling
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
            blend::normal( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity )
        },
        "difference" => {
            blend::difference( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity )
        },
        "multiply" => {
            blend::multiply( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity )
        },
        "overlay" => {
            blend::overlay( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity )
        },
        "screen" => {
            blend::screen( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity )
        },
        _ => {
            panic!(format!("Invalid blend type {}.", &*blend_mode)) // TODO: Proper error handling
        }
    }
}

/// Create a copy of an image as another image.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create image from file
/// let original = Image::from_file("tests/image/sample.jpg");
///
/// // Copy it
/// let copy = editor::copy(&original);
/// ```
pub fn copy(src: &Image) -> Image {
    let mut dest = Image::blank(src.width, src.height);

    for y in 0..dest.height {
        for x in 0..dest.width {
            let rgba = src.get_pixel(x, y);
            let r = rgba[0];
            let g = rgba[1];
            let b = rgba[2];
            let a = rgba[3];
            dest.set_pixel(x, y, &[r, g, b, a]);
        }
    }

    dest
}

/// Crop the image to the given dimension and position.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create image from file
/// let src = Image::from_file("tests/image/sample.gif");
/// 
/// // Crop it
/// let top_left = editor::crop(&src, 250, 128, "top-left", 0, 0);
/// let top_right = editor::crop(&src, 250, 128, "top-right", 0, 0);
/// let center = editor::crop(&src, 250, 128, "center", 0, 0);
///
/// // Save it
/// editor::save(&top_left, "tests/out/test_crop_top_left.png");
/// editor::save(&top_right, "tests/out/test_crop_top_right.png");
/// editor::save(&center, "tests/out/test_crop_center.png");
/// ```
pub fn crop(src: &Image, crop_width: i32, crop_height: i32, position: &str, offset_x: i32, offset_y: i32) -> Image {

    // Turn into positioner struct
    let positioner = Position::new(position, offset_x, offset_y);

    let (offset_x, offset_y) = positioner.get_x_y( src.width, src.height, crop_width, crop_height);
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
            let rgba = src.get_color(offset_x + x, offset_y + y);
            
            dest.set_pixel(x, y, &[rgba.r, rgba.g, rgba.b, rgba.a]);
        }
    }
    dest
}

/// Fill an image with color.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create a 100x100 image
/// let image = Image::blank(100, 100);
///
/// // Fill it with red by passing an RGBA slice
/// let image = editor::fill(&image, &[255, 0, 0, 255]);
///
/// // Save it
/// editor::save(&image, "tests/out/test_fill.png");
/// ```
pub fn fill(src: &Image, color: &[u8]) -> Image {

    let mut dest = Image::blank(src.width, src.height);

    for y in 0..dest.height {
        for x in 0..dest.width {
            dest.set_pixel(x, y, color);
        }
    }

    dest
}

/// Wrapper function for the resizeXXX family of functions. 
/// Resize an image to a given width, height and mode.
pub fn resize(src: &Image, w: i32, h: i32, mode: &str) -> Image {
    
    let dest = match mode {
        "exact" => {
            resize_exact(&src, w, h)
        }
        "exact_width" => {
            resize_exact_width(&src, w)
        }
        "exact_height" => {
            resize_exact_height(&src, h)
        }
        "fit" => {
            resize_fit(&src, w, h)
        },
        "fill" => {
            resize_fill(&src, w, h)
        },
        _ => {
            panic!("Invalid resize mode.")
        },
    };
    
    dest
}

/// Resize image to exact dimensions ignoring aspect ratio. 
/// Useful if you want to force exact width and height.
pub fn resize_exact(src: &Image, w: i32, h: i32) -> Image {

    resample(&src, w, h, "bicubic")

}

/// Resize image to exact height. Width is auto calculated.
/// Useful for creating row of images with the same height.
pub fn resize_exact_height(src: &Image, h: i32) -> Image {

    let width = src.width;
    let height = src.height;
    let ratio = width as f32 / height as f32;

    let resize_height = h;
    let resize_width = (h as f32 * ratio) as i32;

    resample(&src, resize_width, resize_height, "bicubic")
}

/// Resize image to exact width. Height is auto calculated. 
/// Useful for creating column of images with the same width.
pub fn resize_exact_width(src: &Image, w: i32) -> Image {
    let width  = src.width;
    let height = src.height;
    let ratio  = width as f32 / height as f32;

    let resize_width  = w;
    let resize_height = (w as f32 / ratio).round() as i32;

    resample(&src, resize_width, resize_height, "bicubic")
}

/// Resize image to fill all the space in the given dimension. Excess parts are removed.
pub fn resize_fill(src: &Image, w: i32, h: i32) -> Image {
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

    let resized = resample(&src, optimum_width, optimum_height, "bicubic");
    crop(&resized, w, h, "top-left", 0, 0) // Trim excess parts
}

/// Resize an image to fit within the given width and height. 
/// The re-sized image will not exceed the given dimension. 
/// Preserves the aspect ratio.
pub fn resize_fit(src: &Image, w: i32, h: i32) -> Image {
    
    let ratio: f64 = src.width as f64 / src.height as f64;

    // Try basing it on width first
    let mut resize_width  = w;
    let mut resize_height = (w as f64 / ratio).round() as i32;

    if (resize_width > w) || (resize_height > h) { // Oops, either width or height does not fit
        // So base on height instead
        resize_height = h;
        resize_width  = (h as f64 * ratio).round() as i32;
    }

    resample(&src, resize_width, resize_height, "bicubic")
}

/// Save an image into a file.
pub fn save(image: &Image, out: &str){
    image::save_buffer(&Path::new(out), &image.pixels, image.width as u32, image.height as u32, image::RGBA(8)).unwrap();
}


// Private functions

// Interpolate using nearest neighbor.
fn interpolate_nearest(src: &Image, w: i32, h: i32) -> Image {
    
    let x_ratio: f64 = src.width as f64 / w as f64;
    let y_ratio: f64 = src.height as f64 / h as f64;
    
    let mut dest = Image::blank(w, h);
    for y in 0..h {
        for x in 0..w {

            let px: i32 = ( x as f64 * x_ratio ).floor() as i32;
            let py: i32 = ( y as f64 * y_ratio ).floor() as i32;
            let p = src.get_pixel(px, py);
            let r = p[0];
            let g = p[1];
            let b = p[2];
            let a = p[3];
            dest.set_pixel(x, y, &[r,g,b,a]);
        }
    }
    
    dest
}

// Resample an image into a new size.
fn resample(src: &Image, w: i32, h: i32, interpolation: &str) -> Image {
    
    let dest = match interpolation {
        "bilinear" => {
            interpolate_nearest(&src, w, h) // TODO
        },
        "bicubic" => {
            interpolate_nearest(&src, w, h) // TODO
        },
        _ => {
            interpolate_nearest(&src, w, h)
        },
    };
    
    dest
}
