//!  A module for interpolating pixels.

// crates
extern crate image;

// from rust
use std::cmp;

// from external crate


// from local crate
use Image;
use Color;

/// Resample an image into a new size using a given interpolation method.
pub fn resample<'a>(mut src: &'a mut Image, w: i32, h: i32, interpolation: &str) -> Result<(), String> {
    
    match interpolation {
        "bilinear" => {
            try!(bilinear(&mut src, w, h));
            Ok(())
        },
        "bicubic" => {
            try!(bilinear(&mut src, w, h)); // TODO: bicubic
            Ok(())
        },
        "nearest" => {
            try!(nearest(&mut src, w, h));
            Ok(())
        },
        _ => {
            Err(format!("Invalid interpolation '{}'", interpolation))
        }
    }
}

/// Interpolate using nearest neighbor.
pub fn nearest<'a>(mut src: &'a mut Image, w: i32, h: i32) -> Result<(), String> {
    
    let x_ratio: f64 = src.width as f64 / w as f64;
    let y_ratio: f64 = src.height as f64 / h as f64;
    
    let mut dest = Image::blank(w, h);
    for y in 0..h {
        for x in 0..w {

            let px: i32 = ( x as f64 * x_ratio ).floor() as i32;
            let py: i32 = ( y as f64 * y_ratio ).floor() as i32;
            let pixel = try!(src.get_pixel(px, py));
            
            try!(dest.set_pixel(x, y, pixel));
        }
    }
    src.width = dest.width;
    src.height = dest.height;
    src.bytes = dest.bytes;

    Ok(())
}

/// Interpolate using linear function.
pub fn bilinear<'a>(mut src: &'a mut Image, w2: i32, h2: i32) -> Result<(), String> {
    
    try!(bilinear_width(&mut src, w2));
    try!(bilinear_height(&mut src, h2));
    
    Ok(())
}

// Private functions

/// Interpolate the width using linear function.
fn bilinear_width<'a>(mut src: &'a mut Image, w2: i32) -> Result<(), String> {
    
    let w1 = src.width;
    let h1 = src.height;

    let x_ratio: f64 = w1 as f64 / w2 as f64;
    
    let mut dest = Image::blank(w2, h1);
    
    let offset_x = (w2 / w1 / 2) as i32;

    let x_start = 0 - offset_x;
    let x_end = w2 - offset_x;

    for y in 0..h1 {
        for x in x_start..x_end {
            
            let mut src_x = x as f64 * x_ratio;
            if src_x < 0.0 {
                src_x = 0.0; // limit lower bound to 0
            }

            let src_x_int = (src_x).floor() as i32;

            let src_x_int2 = cmp::min(src_x_int + 1, w1-1); // limit range withn $w1-1

            // limit range from 0 - 1
            let t_x = src_x - src_x_int as f64;

            let src_color1 = try!(src.get_pixel(src_x_int, y));
            let src_color2 = try!(src.get_pixel(src_x_int2, y));
            
            // red
            let red = _lerp(src_color1.r, src_color2.r, t_x);

            // green
            let green = _lerp(src_color1.g, src_color2.g, t_x);

            // blue
            let blue = _lerp(src_color1.b, src_color2.b, t_x);

            // alpha
            let alpha = _lerp(src_color1.a, src_color2.a, t_x);

            try!(dest.set_pixel(x+offset_x, y, Color::rgba(red, green, blue, alpha)));

        }
    }
    src.width = dest.width;     
    src.height = dest.height;     
    src.bytes = dest.bytes;
    
    Ok(())
}

/// Interpolate the height using linear function.
fn bilinear_height<'a>(mut src: &'a mut Image, h2: i32) -> Result<(), String> {
    
    let w1 = src.width;
    let h1 = src.height;

    let y_ratio: f64 = h1 as f64 / h2 as f64;
    
    let mut dest = Image::blank(w1, h2);
    
    let offset_y = (h2 / h1 / 2) as i32;

    let y_start = 0 - offset_y;
    let y_end = h2 - offset_y;

    for x in 0..w1 {
        for y in y_start..y_end {
            
            let mut src_y = y as f64 * y_ratio;

            if src_y < 0.0 {
                src_y = 0.0; // limit lower bound to 0
            }

            let src_y_int = (src_y).floor() as i32;

            let src_y_int2 = cmp::min(src_y_int + 1, h1-1); // limit range withn $h1-1

            // limit range from 0 - 1
            let t_y = src_y - src_y_int as f64;

            let src_color1 = try!(src.get_pixel(x, src_y_int));
            let src_color2 = try!(src.get_pixel(x, src_y_int2));
            
            // red
            let red = _lerp(src_color1.r, src_color2.r, t_y);

            // green
            let green = _lerp(src_color1.g, src_color2.g, t_y);

            // blue
            let blue = _lerp(src_color1.b, src_color2.b, t_y);

            // alpha
            let alpha = _lerp(src_color1.a, src_color2.a, t_y);

            try!(dest.set_pixel(x, y+offset_y, Color::rgba(red, green, blue, alpha)));

        }
    }
    src.width = dest.width;
    src.height = dest.height;
    src.bytes = dest.bytes;

    Ok(())
}

// Simple linear function
fn _lerp(a:u8, b:u8, t:f64) -> u8{

    let a = a as f64;
    let b = b as f64;

    (a + (t * (b - a))) as u8

}

// Linear function using difference
fn _bilinear(a: u8, b: u8, c: u8, d: u8, x_diff: f64, y_diff: f64) -> u8 {
    // Y = A(1-w)(1-h) + B(w)(1-h) + C(h)(1-w) + Dwh
    (
        a as f64 * (1.0 - x_diff) * (1.0 - y_diff) + b as f64 * (x_diff) * (1.0 - y_diff) +
        c as f64  * (y_diff) * (1.0 - x_diff) + d as f64  * (x_diff * y_diff)
    ) as u8
}