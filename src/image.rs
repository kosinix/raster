extern crate image;

// from rust
use std::path::Path;

// from external crate
use self::image::GenericImage;


pub struct Image<'a> {
    pub format: &'a str,    // is a Copy type. No need for borrowing.
    pub width: u32,         // is a Copy type. No need for borrowing.
    pub height: u32,        // is a Copy type. No need for borrowing.
    pub pixels: Vec<u8>,    // Store pixels in RGBA format
}

impl<'a> Image<'a> {
    
    /// Create an image from file
    pub fn from_file(file: &'a str) -> Image {
        
        let src = image::open(file).unwrap(); // Returns image::DynamicImage
        let (w,h) = src.dimensions();
        let mut pixels = Vec::new();
        for y in 0..h {
            for x in 0..w {
                let p = src.get_pixel(x, y);
                pixels.push(p.data[0]);
                pixels.push(p.data[1]);
                pixels.push(p.data[2]);
                pixels.push(p.data[3]);
            }
        }
        let path = Path::new(file);
        Image{ 
            format: Image::ext_to_format(path.extension().unwrap().to_str().unwrap()),
            width: w,
            height: h,
            pixels: pixels
        }
    }
    
    pub fn blank(w:u32, h:u32) -> Image<'a> {
        
        let mut pixels = Vec::new();
        for _ in 0..h {
            for _ in 0..w {
                pixels.push(255);
                pixels.push(255);
                pixels.push(255);
                pixels.push(255);
            }
        }
        Image { 
            format: "UNKNOWN",
            width: w,
            height: h,
            pixels: pixels
        }
    }

    /// Get pixels vector
    /// TODO: sanity checks
    pub fn get_pixel(&self, x: u32, y:u32) -> &[u8] {
        let rgba = 4;
        let sx = (y * &self.width) + x;
        let start = sx * rgba;
        let end = start + rgba;
        
        &self.pixels[start as usize..end as usize]
    }

    /// Get pixels vector
    /// TODO: sanity checks
    pub fn set_pixel(&mut self, x: u32, y:u32, pixel: &[u8]) {
        let rgba = 4; // length
        let sx = (y * &self.width) + x;
        let start = sx * rgba;
        
        self.pixels[start as usize] = pixel[0];
        self.pixels[start as usize + 1] = pixel[1];
        self.pixels[start as usize + 2] = pixel[2];
        self.pixels[start as usize + 3] = pixel[3];
    }

    pub fn pixels(&self) -> &Vec<u8> {
        &self.pixels
    }

    pub fn set_pixels(&mut self, pixels: Vec<u8>) {
        self.pixels = pixels;
    }

    pub fn set_width(&mut self, w: u32) {
        self.width = w;
    }

    pub fn set_height(&mut self, h: u32) {
        self.height = h;
    }

    /// Get file extension
    fn ext_to_format(ext: &str) -> &str {
        let lowercase = ext.to_string().to_lowercase();
        let ext = &*lowercase;
        match ext {
            "jpg" => "JPEG",
            "png" => "PNG",
            "gif" => "GIF",
            _ => "UNKNOWN", 
        }
    }
}