//!  A module for blending 2 images.

// crates
extern crate image;

// from rust


// from external crate


// from local crate
use editor;
use image::Image;


pub fn multiply(image1: &Image, image2: &Image, loop_start_y:i32, loop_end_y:i32, loop_start_x:i32, loop_end_x:i32, offset_x:i32, offset_y:i32, opacity:f32) -> Image {

    let mut canvas = editor::copy(&image1);

    for y in loop_start_y..loop_end_y {
        for x in loop_start_x..loop_end_x {
            let canvas_x = x + offset_x;
            let canvas_y = y + offset_y;
            let rgba1 = image1.get_pixel(canvas_x, canvas_y);
            let a1 = rgba1[3] as f32 / 255.0; // convert
            let r1 = rgba1[0] as f32 * a1;
            let g1 = rgba1[1] as f32 * a1;
            let b1 = rgba1[2] as f32 * a1;
            
            let rgba2 = image2.get_pixel(x, y);
            let a2 = rgba2[3] as f32 / 255.0 * opacity; // convert
            let r2 = rgba2[0] as f32 * a2;
            let g2 = rgba2[1] as f32 * a2;
            let b2 = rgba2[2] as f32 * a2;
            
            let r3 = r1 * r2 / 255.0;
            let g3 = g1 * g2 / 255.0;
            let b3 = b1 * b2 / 255.0;
            let a3 = rgba2[3] as f32 * opacity + ((1.0-opacity) * rgba1[3] as f32); // Blending with alpha is bork
            
            canvas.set_pixel(canvas_x, canvas_y, &[r3 as u8, g3 as u8, b3 as u8, a3 as u8]);
        }
    }
    
    canvas
}

// See https://en.wikipedia.org/wiki/Alpha_compositing
// See http://stackoverflow.com/questions/9014729/manually-alpha-blending-an-rgba-pixel-with-an-rgb-pixel
pub fn normal(image1: &Image, image2: &Image, loop_start_y:i32, loop_end_y:i32, loop_start_x:i32, loop_end_x:i32, offset_x:i32, offset_y:i32, opacity:f32) -> Image {

    let mut canvas = editor::copy(&image1);

    for y in loop_start_y..loop_end_y {
        for x in loop_start_x..loop_end_x {
            let canvas_x = x + offset_x;
            let canvas_y = y + offset_y;
            
            let rgba1 = image1.get_pixel(canvas_x, canvas_y);
            let r1 = rgba1[0] as f32;
            let g1 = rgba1[1] as f32;
            let b1 = rgba1[2] as f32;
            let a1 = rgba1[3] as f32 / 255.0; // convert

            let rgba2 = image2.get_pixel(x, y);
            let r2 = rgba2[0] as f32;
            let g2 = rgba2[1] as f32;
            let b2 = rgba2[2] as f32;
            let a2 = rgba2[3] as f32 / 255.0; // convert 

            let a2 = a2 * opacity;
            let r3 = (r2 * a2) + ((r1 * a1) * (1.0 - a2));
            let g3 = (g2 * a2) + ((g1 * a1) * (1.0 - a2));
            let b3 = (b2 * a2) + ((b1 * a1) * (1.0 - a2));
            
            let a3 = (a2 + (a1 * ( 1.0 - a2))) * 255.0;
            
            canvas.set_pixel(canvas_x, canvas_y, &[r3 as u8, g3 as u8, b3 as u8, a3 as u8]);
        }
    }
    
    canvas
}

pub fn overlay(image1: &Image, image2: &Image, loop_start_y:i32, loop_end_y:i32, loop_start_x:i32, loop_end_x:i32, offset_x:i32, offset_y:i32, opacity:f32) -> Image {

    let mut canvas = editor::copy(&image1);

    for y in loop_start_y..loop_end_y {
        for x in loop_start_x..loop_end_x {
            let canvas_x = x + offset_x;
            let canvas_y = y + offset_y;
            let rgba1 = image1.get_pixel(canvas_x, canvas_y);
            let mut r1 = rgba1[0] as f32;
            let mut g1 = rgba1[1] as f32;
            let mut b1 = rgba1[2] as f32;

            let rgba2 = image2.get_pixel(x, y);
            let mut r2 = rgba2[0] as f32;
            let mut g2 = rgba2[1] as f32;
            let mut b2 = rgba2[2] as f32;
            
            r1 /= 255.0;
            r2 /= 255.0;
            let mut r3 = 1.0 - (2.0 *(1.0-r1)) * (1.0-r2);
            if r1 < 0.5 {
                r3 = 2.0 * (r1 * r2);
            }

            g1 /= 255.0;
            g2 /= 255.0;
            let mut g3 = 1.0 - (2.0 *(1.0-g1)) * (1.0-g2);
            if g1 < 0.5 {
                g3 = 2.0 * (g1 * g2);
            }

            b1 /= 255.0;
            b2 /= 255.0;
            let mut b3 = 1.0 - (2.0 *(1.0-b1)) * (1.0-b2);
            if b1 < 0.5 {
                b3 = 2.0 * (b1 * b2);
            }
            
            let a3 = opacity * 255.0;
            
            r3 *= 255.0;
            g3 *= 255.0;
            b3 *= 255.0;
            canvas.set_pixel(canvas_x, canvas_y, &[r3 as u8, g3 as u8, b3 as u8, a3 as u8]);
        }
    }
    
    canvas
}

pub fn screen(image1: &Image, image2: &Image, loop_start_y:i32, loop_end_y:i32, loop_start_x:i32, loop_end_x:i32, offset_x:i32, offset_y:i32, opacity:f32) -> Image {

    let mut canvas = editor::copy(&image1);

    for y in loop_start_y..loop_end_y {
        for x in loop_start_x..loop_end_x {
            let canvas_x = x + offset_x;
            let canvas_y = y + offset_y;
            let rgba1 = image1.get_pixel(canvas_x, canvas_y);
            let a1 = rgba1[3] as f32 / 255.0;
            let r1 = rgba1[0] as f32 * a1;
            let g1 = rgba1[1] as f32 * a1;
            let b1 = rgba1[2] as f32 * a1;

            let rgba2 = image2.get_pixel(x, y);
            let a2 = rgba2[3] as f32 / 255.0 * opacity;
            let r2 = rgba2[0] as f32 * a2;
            let g2 = rgba2[1] as f32 * a2;
            let b2 = rgba2[2] as f32 * a2;
            
            let r3 = 255.0 - ( ( 255.0 - r1 ) * ( 255.0 - r2 ) ) / 255.0;
            let g3 = 255.0 - ( ( 255.0 - g1 ) * ( 255.0 - g2 ) ) / 255.0;
            let b3 = 255.0 - ( ( 255.0 - b1 ) * ( 255.0 - b2 ) ) / 255.0;
            let a3 = (a2 + (a1 * ( 1.0 - a2))) * 255.0;

            canvas.set_pixel(canvas_x, canvas_y, &[r3 as u8, g3 as u8, b3 as u8, a3 as u8]);
        }
    }
    
    canvas
}