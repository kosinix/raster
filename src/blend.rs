//!  A module for blending 2 images.

// See https://en.wikipedia.org/wiki/Alpha_compositing

// crates
extern crate image;

// from rust


// from external crate


// from local crate
use editor;
use Image;
use color::Color;

pub fn difference(image1: &Image, image2: &Image, loop_start_y:i32, loop_end_y:i32, loop_start_x:i32, loop_end_x:i32, offset_x:i32, offset_y:i32, opacity:f32) -> Result<Image, String> {

    let mut canvas = editor::clone(&image1);

    for y in loop_start_y..loop_end_y {
        for x in loop_start_x..loop_end_x {
            let canvas_x = x + offset_x;
            let canvas_y = y + offset_y;
            let rgba1 = try!(image1.get_pixel(canvas_x, canvas_y));
            let a1 = rgba1.a as f32 / 255.0; // convert to 0.0 - 1.0
            let r1 = rgba1.r as f32 * a1;
            let g1 = rgba1.g as f32 * a1;
            let b1 = rgba1.b as f32 * a1;
            
            let rgba2 = try!(image2.get_pixel(x, y));
            let a2 = rgba2.a as f32 / 255.0 * opacity; // convert to 0.0 - 1.0
            let r2 = rgba2.r as f32;
            let g2 = rgba2.g as f32;
            let b2 = rgba2.b as f32;
            
            let r3 = ch_alpha_f(r1, r2, "difference", a2);
            let g3 = ch_alpha_f(g1, g2, "difference", a2);
            let b3 = ch_alpha_f(b1, b2, "difference", a2);
            let a3 = 255;
            
            try!(canvas.set_pixel(canvas_x, canvas_y, Color::rgba(r3 as u8, g3 as u8, b3 as u8, a3 as u8)));
        }
    }
    
    Ok(canvas)
}

pub fn multiply(image1: &Image, image2: &Image, loop_start_y:i32, loop_end_y:i32, loop_start_x:i32, loop_end_x:i32, offset_x:i32, offset_y:i32, opacity:f32) -> Result<Image, String> {

    let mut canvas = editor::clone(&image1);

    for y in loop_start_y..loop_end_y {
        for x in loop_start_x..loop_end_x {
            let canvas_x = x + offset_x;
            let canvas_y = y + offset_y;
            let rgba1 = try!(image1.get_pixel(canvas_x, canvas_y));
            let a1 = rgba1.a as f32 / 255.0; // convert to 0.0 - 1.0
            let r1 = rgba1.r as f32 * a1;
            let g1 = rgba1.g as f32 * a1;
            let b1 = rgba1.b as f32 * a1;
            
            let rgba2 = try!(image2.get_pixel(x, y));
            let a2 = rgba2.a as f32 / 255.0 * opacity; // convert to 0.0 - 1.0
            let r2 = rgba2.r as f32;
            let g2 = rgba2.g as f32;
            let b2 = rgba2.b as f32;
            
            let r3 = ch_alpha_f(r1, r2, "multiply", a2);
            let g3 = ch_alpha_f(g1, g2, "multiply", a2);
            let b3 = ch_alpha_f(b1, b2, "multiply", a2);
            let a3 = 255;
            
            try!(canvas.set_pixel(canvas_x, canvas_y, Color::rgba(r3 as u8, g3 as u8, b3 as u8, a3 as u8)));
        }
    }
    
    Ok(canvas)
}

pub fn normal(image1: &Image, image2: &Image, loop_start_y:i32, loop_end_y:i32, loop_start_x:i32, loop_end_x:i32, offset_x:i32, offset_y:i32, opacity:f32) -> Result<Image, String> {

    let mut canvas = editor::clone(&image1);

    for y in loop_start_y..loop_end_y {
        for x in loop_start_x..loop_end_x {
            let canvas_x = x + offset_x;
            let canvas_y = y + offset_y;
            let color1 = try!(image1.get_pixel(canvas_x, canvas_y));
            let a1 = color1.a as f32 / 255.0; // convert to 0.0 - 1.0
            let r1 = color1.r as f32 * a1;
            let g1 = color1.g as f32 * a1;
            let b1 = color1.b as f32 * a1;
            
            let color2 = try!(image2.get_pixel(x, y));
            let a2 = color2.a as f32 / 255.0 * opacity; // convert to 0.0 - 1.0
            let r2 = color2.r as f32;
            let g2 = color2.g as f32;
            let b2 = color2.b as f32;
            
            let r3 = (a2 * r2) + ((1.0 - a2) * r1);
            let g3 = (a2 * g2) + ((1.0 - a2) * g1);
            let b3 = (a2 * b2) + ((1.0 - a2) * b1);
            let a3 = 255;
            
            try!(canvas.set_pixel(canvas_x, canvas_y, Color::rgba(r3 as u8, g3 as u8, b3 as u8, a3 as u8)));
        }
    }
    
    Ok(canvas)
}

pub fn overlay(image1: &Image, image2: &Image, loop_start_y:i32, loop_end_y:i32, loop_start_x:i32, loop_end_x:i32, offset_x:i32, offset_y:i32, opacity:f32) -> Result<Image, String> {

    let mut canvas = editor::clone(&image1);

    for y in loop_start_y..loop_end_y {
        for x in loop_start_x..loop_end_x {
            let canvas_x = x + offset_x;
            let canvas_y = y + offset_y;
            let rgba1 = try!(image1.get_pixel(canvas_x, canvas_y));
            let a1 = rgba1.a as f32 / 255.0; // convert to 0.0 - 1.0
            let r1 = rgba1.r as f32 * a1;
            let g1 = rgba1.g as f32 * a1;
            let b1 = rgba1.b as f32 * a1;
            
            let rgba2 = try!(image2.get_pixel(x, y));
            let a2 = rgba2.a as f32 / 255.0 * opacity; // convert to 0.0 - 1.0
            let r2 = rgba2.r as f32;
            let g2 = rgba2.g as f32;
            let b2 = rgba2.b as f32;
            
            let r3 = ch_alpha_f(r1, r2, "overlay", a2);
            let g3 = ch_alpha_f(g1, g2, "overlay", a2);
            let b3 = ch_alpha_f(b1, b2, "overlay", a2);
            let a3 = 255;

            try!(canvas.set_pixel(canvas_x, canvas_y, Color::rgba(r3 as u8, g3 as u8, b3 as u8, a3 as u8)));
        }
    }
    
    Ok(canvas)
}

pub fn screen(image1: &Image, image2: &Image, loop_start_y:i32, loop_end_y:i32, loop_start_x:i32, loop_end_x:i32, offset_x:i32, offset_y:i32, opacity:f32) -> Result<Image, String> {

    let mut canvas = editor::clone(&image1);

    for y in loop_start_y..loop_end_y {
        for x in loop_start_x..loop_end_x {
            let canvas_x = x + offset_x;
            let canvas_y = y + offset_y;
            let rgba1 = try!(image1.get_pixel(canvas_x, canvas_y));
            let a1 = rgba1.a as f32 / 255.0; // convert to 0.0 - 1.0
            let r1 = rgba1.r as f32 * a1;
            let g1 = rgba1.g as f32 * a1;
            let b1 = rgba1.b as f32 * a1;
            
            let rgba2 = try!(image2.get_pixel(x, y));
            let a2 = rgba2.a as f32 / 255.0 * opacity; // convert to 0.0 - 1.0
            let r2 = rgba2.r as f32;
            let g2 = rgba2.g as f32;
            let b2 = rgba2.b as f32;
            
            let r3 = ch_alpha_f(r1, r2, "screen", a2);
            let g3 = ch_alpha_f(g1, g2, "screen", a2);
            let b3 = ch_alpha_f(b1, b2, "screen", a2);
            let a3 = 255;

            try!(canvas.set_pixel(canvas_x, canvas_y, Color::rgba(r3 as u8, g3 as u8, b3 as u8, a3 as u8)));
        }
    }
    
    Ok(canvas)
}

// PRIVATE FNs
// base, top 0.0 - 255.0  
// opacity 0.0 - 1.0

fn ch_alpha_f(base: f32, top: f32, f: &str, opacity: f32) -> f32 {
    match f {
        "difference" => ch_alpha( base, ch_difference( base, top ), opacity ),
        "multiply" => ch_alpha( base, ch_multiply( base, top ), opacity ),
        "overlay" => ch_alpha( base, ch_overlay( base, top ), opacity ),
        "screen" => ch_alpha( base, ch_screen( base, top ), opacity ),
        _ => panic!("Invalid blend function.") // TODO:
    }
}

fn ch_alpha(base: f32, top: f32, opacity: f32) -> f32 {
    (opacity * top) + ((1.0 - opacity) * base)
}

fn ch_difference(base: f32, top: f32) -> f32 {
    (base - top).abs()
}

fn ch_multiply(base: f32, top: f32) -> f32 {
    (base * top) / 255.0
}

fn ch_overlay(base: f32, top: f32) -> f32 {
    if base < 128.0 {
        2.0 * base * top / 255.0
    } else {
        255.0 - ((2.0 * (255.0 - base)) * (255.0 - top) / 255.0)
    }
}

fn ch_screen(base: f32, top:f32) -> f32 {
    255.0 - (((255.0 - base) * (255.0 - top)) / 255.0)
}

