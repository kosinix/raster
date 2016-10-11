extern crate image;

// from rust
use std::path::Path;

// from external crate
use self::image::ImageBuffer;
use self::image::GenericImage;
use self::image::DynamicImage;
use self::image::RgbImage;

// from local crate
use image::Image;

pub fn copy() {
    println!("copying image.png");
    let img1 = image::open("image.png").unwrap(); // Returns image::DynamicImage
    let (w1, h1) = img1.dimensions();

    let mut img2 = ImageBuffer::new(w1, h1);
    for y in 0..h1 {
        for x in 0..w1 {
            let p = img1.get_pixel(x, y);
            //println!("pixel {}{} {:?}", x,y,p);
            img2.put_pixel(x, y, p);
        }
    }
    println!("saving to image2.png");
    let _ = img2.save(&Path::new("image2.png"));
    // // Iterate over the coordiantes and pixels of the image
    // for (x, y, pixel) in img.enumerate_pixels_mut() {
        
    //     println!("{:?}", pixel);

    //     // Create an 8bit pixel of type Luma and value i
    //     // and assign in to the pixel at position (x, y)
    //     *pixel = image::Rgb([255,0,0]);

    // }
    // println!("{:?}", img.into_raw());
    // let mut img = image::open("image.png").unwrap(); // Returns image::DynamicImage
    
    // img = img.flip_horizontal();
    // let (width, height) = img.dimensions();

    // for y in 0..height {
    //     for x in 0..width {
    //         let p = img.get_pixel(x, y);
    //         println!("pixel {}{} {:?}", x,y,p);
    //         img.put_pixel(y, width - 1 - x, p);
    //     }
    // }
    // for e in img.pixels() {
    //     let (_, _, color) = e;

        
    //     println!("Pixel colour {:?}", color);
    //     println!("Pixel colour {}", color.data[3]);
    // }

    // let p = image::Rgba { data: [0, 0, 255, 255] };
    // img.put_pixel(0,0,p);
    // println!("pixel 0 0 {:?}", img.get_pixel(0,0));
    // println!("Dimensions {:?}", img.dimensions());

    // let ref mut fout = File::create(&Path::new("image2.png")).unwrap();
    // let _ = img.save(fout, image::PNG).unwrap();

    // let buffer: &[u8] = &[0, 0, 0, 255, 0, 0]; // Generate the image data

    // // Save the buffer as "image.png"
    // image::save_buffer(&Path::new("image2.png"), buffer, 2, 1, image::RGB(8)).unwrap()
}

pub fn save(image: &Image, out: &str){
    println!("Save {} to {}", image.format, out);
    image::save_buffer(&Path::new(out), image.pixels(), image.width, image.height, image::RGBA(8)).unwrap();
}

pub fn slice<'a>(src: &mut Image<'a>, ox: u32, oy: u32, w2: u32, h2: u32) -> Image<'a> {
    let src_w = src.width - ox; // Subtract x offset
    let src_h = src.height - oy; // Subtract y offset

    let mut dest = ImageBuffer::new(w2, h2);

    let x_ratio: f64 = (src_w as f64 / w2 as f64);
    let y_ratio: f64 = (src_h as f64 / h2 as f64);

    for y in 0..h2 {
        for x in 0..w2 {
            let px: u32 = ( x as f64 * x_ratio ).floor() as u32;
            let py: u32 = ( y as f64 * y_ratio ).floor() as u32;
            let p = src.get_pixel(px+ox, py+oy);
            let r = p[0];
            let g = p[1];
            let b = p[2];
            let a = p[3];
            dest.put_pixel(x, y, image::Rgba([r, g, b, a]));
        }
    }
    Image { 
        format: src.format,
        width: w2,
        height: h2,
        pixels: dest.into_raw()
    }
}

pub fn resize<'a>(src: &Image<'a>, w:u32, h:u32) -> Image<'a> {
    let img: RgbImage = ImageBuffer::new(src.width, src.height);
    let result = self::image::imageops::resize(&img, w, h, self::image::FilterType::Lanczos3);
    Image { 
        format: src.format,
        width: w,
        height: h,
        pixels: result.into_raw()
    }
}

pub fn resample<'a>(src: &Image<'a>, w: u32, h: u32) -> Image<'a> {
    
    let x_ratio: f64 = src.width as f64 / w as f64;
    let y_ratio: f64 = src.height as f64 / h as f64;
    
    let mut dest = ImageBuffer::new(w, h);
    for y in 0..h {
        for x in 0..w {

            let px: u32 = ( x as f64 * x_ratio ).floor() as u32;
            let py: u32 = ( y as f64 * y_ratio ).floor() as u32;
            let p = src.get_pixel(px, py);
            let r = p[0];
            let g = p[1];
            let b = p[2];
            let a = p[3];
            dest.put_pixel(x, y, image::Rgba([r, g, b, a]));
        }
    }
    
    Image { 
        format: src.format,
        width: w,
        height: h,
        pixels: dest.into_raw()
    }
}

pub fn crop(){

}

pub fn resizeFit(){

}

pub fn resizeFill(){

}

pub fn blend(){
    
}