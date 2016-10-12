extern crate image;

// from rust
use std::path::Path;

// from external crate
// use self::image::ImageBuffer;
// use self::image::GenericImage;

// from local crate
use image::Image;


pub fn save(image: &Image, out: &str){
    println!("Save {} to {}", image.format, out);
    image::save_buffer(&Path::new(out), image.pixels(), image.width, image.height, image::RGBA(8)).unwrap();
}

pub fn slice<'a>(src: &mut Image<'a>, ox: u32, oy: u32, w2: u32, h2: u32) -> Image<'a> {
    let src_w = src.width - ox; // Subtract x offset
    let src_h = src.height - oy; // Subtract y offset

    let mut dest = Image::blank(w2, h2);

    let x_ratio: f64 = src_w as f64 / w2 as f64;
    let y_ratio: f64 = src_h as f64 / h2 as f64;

    for y in 0..h2 {
        for x in 0..w2 {
            let px: u32 = ( x as f64 * x_ratio ).floor() as u32;
            let py: u32 = ( y as f64 * y_ratio ).floor() as u32;
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



pub fn resample<'a>(src: &Image<'a>, w: u32, h: u32) -> Image<'a> {
    
    let x_ratio: f64 = src.width as f64 / w as f64;
    let y_ratio: f64 = src.height as f64 / h as f64;
    
    let mut dest = Image::blank(w, h);
    for y in 0..h {
        for x in 0..w {

            let px: u32 = ( x as f64 * x_ratio ).floor() as u32;
            let py: u32 = ( y as f64 * y_ratio ).floor() as u32;
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

pub fn crop(){

}

pub fn resize_fit(){

}

pub fn resize_fill(){

}

pub fn blend(){
    
}