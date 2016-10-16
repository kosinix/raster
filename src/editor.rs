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

pub fn blend<'a>(image1: &Image, image2: &Image, blend_mode: &str, alpha: f32, position: &str, offset_x: i32, offset_y: i32) -> Image {
    
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
            blend::normal( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, alpha )
        },
        "multiply" => {
            blend::multiply( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, alpha )
        },
        "overlay" => {
            blend::overlay( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, alpha )
        },
        "screen" => {
            blend::screen( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, alpha )
        },
        _ => {
            panic!(format!("Invalid blend type {}.", &*blend_mode)) // TODO: Error handling
        }
    }
}

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

pub fn fill(src: &Image, color: &[u8]) -> Image {

    let mut dest = Image::blank(src.width, src.height);

    for y in 0..dest.height {
        for x in 0..dest.width {
            dest.set_pixel(x, y, color);
        }
    }

    dest
}

pub fn resize(src: &Image, w: i32, h: i32, mode: &str) -> Image {
    
    let dest = match mode {
        "exact" => {
            resize_exact(&src, w, h)
        }
        "fit" => {
            resample(&src, w, h, "bicubic")
        },
        "fill" => {
            resample(&src, w, h, "bicubic")
        },
        _ => {
            resize_fit(&src, w, h)
        },
    };
    
    dest
}

pub fn save(image: &Image, out: &str){
    image::save_buffer(&Path::new(out), &image.pixels, image.width as u32, image.height as u32, image::RGBA(8)).unwrap();
}

pub fn slice(src: &mut Image, ox: i32, oy: i32, w2: i32, h2: i32) -> Image {
    let src_w = src.width - ox; // Subtract x offset
    let src_h = src.height - oy; // Subtract y offset

    let mut dest = Image::blank(w2, h2);

    let x_ratio: f64 = src_w as f64 / w2 as f64;
    let y_ratio: f64 = src_h as f64 / h2 as f64;

    for y in 0..h2 {
        for x in 0..w2 {
            let px: i32 = ( x as f64 * x_ratio ).floor() as i32;
            let py: i32 = ( y as f64 * y_ratio ).floor() as i32;
            let p = src.get_pixel(px+ox, py+oy);
            let r = p[0];
            let g = p[1];
            let b = p[2];
            let a = p[3];
            dest.set_pixel(x, y, &[r,g,b,a]);
        }
    }
    
    dest
}



pub fn resample(src: &Image, w: i32, h: i32, interpolation: &str) -> Image {
    
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

pub fn crop(){

}

pub fn resize_fit(src: &Image, w: i32, h: i32) -> Image {
    
    let ratio: f64 = src.width as f64 / src.height as f64;

    // Try basing it on width first
    let mut resize_width  = w;
    let mut resize_height = (w as f64 / ratio).round() as i32;

    if (resize_width > w) || (resize_height > h) { // Oops, either witdh or height does not fit
        // So base on height instead
        resize_height = h;
        resize_width  = (h as f64 * ratio).round() as i32;
    }

    resample(&src, resize_width, resize_height, "bicubic")
}

pub fn resize_exact(src: &Image, w: i32, h: i32) -> Image {
    
    resample(&src, w, h, "bicubic")
}

pub fn resize_fill(){

}

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


