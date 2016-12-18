//!  A module for filtering pixels.

// crates


// from rust
use std::cmp;

// from external crate


// from local crate
use editor;
use Image;
use Color;

/// Apply box or Gaussian blur.
///
/// # Examples
/// ### Box Blur
///
/// ```
/// use raster::filter;
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::blur(&mut image, "box").unwrap();
/// raster::save(&image, "tests/out/test_filter_box_blur.jpg");
/// ```
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
/// 
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_box_blur.jpg)
///
/// ### Gaussian Blur
///
/// ```
/// use raster::filter;
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::blur(&mut image, "gaussian").unwrap();
/// raster::save(&image, "tests/out/test_filter_gaussian_blur.jpg");
/// ```
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
/// 
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_gaussian_blur.jpg)
///
pub fn blur<'a>(mut src: &'a mut Image, mode: &str) -> Result<(), String>{

    match mode {
        "box" => {
            try!(blur_box(&mut src));
            Ok(())
        },
        "gaussian" => {
            try!(blur_gaussian(&mut src));
            Ok(())
        },
        _ => {
            Err(format!("Invalid mode '{}'", mode))
        }
    }
}

/// Apply brightness. 
///
/// A brightness of < 0.0 will darken the image and brightness of > 1.0 will lighten it.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::brightness(&mut image, 1.5).unwrap();
/// raster::save(&image, "tests/out/test_filter_brightness.jpg");
/// ```
///
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
/// 
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_brightness.jpg)
///
pub fn brightness(mut src: &mut Image, factor: f32) -> Result<(), String>{
    let w: i32 = src.width;
    let h: i32 = src.height;

    // if gamma < 0.01 || gamma > 9.99{
    //     return Err(format!("Incorrect gamma value {}. Must be in range 0.01 - 9.99.", gamma));
    // }
    // let factor = 255.0 * factor;

    for y in 0..h {
        for x in 0..w {
            
            let p = try!(src.get_pixel(x, y));
            let r = cmp::max(0, cmp::min(255, (p.r as f32 * factor) as i32));
            let g = cmp::max(0, cmp::min(255, (p.g as f32 * factor) as i32));
            let b = cmp::max(0, cmp::min(255, (p.b as f32 * factor) as i32));
            let a = cmp::max(0, cmp::min(255, (p.a as f32 * factor) as i32)); // TODO: Should alpha be included?
            
            try!(src.set_pixel(x, y, Color::rgba(r as u8, g as u8, b as u8, a as u8)));
            
        }
    }
    
    Ok(())
}

/// Apply a convolvution matrix. 
///
/// The divisor is applied as the last step of convolution.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// let matrix: [[i32; 3]; 3] = [
///     [0, 0, 0],
///     [0, 1, 0],
///     [0, 0, 0]
/// ];
/// filter::convolve(&mut image, matrix, 1).unwrap();
/// raster::save(&image, "tests/out/test_filter_convolve.jpg");
/// ```
pub fn convolve(src: &mut Image, matrix: [[i32; 3]; 3], divisor: i32) -> Result<(), String> {
    
    let w: i32 = src.width;
    let h: i32 = src.height;
    let m_size = 3; // Matrix size
    
    let copy = editor::clone(&src); // Create a copy as input of pixels

    for y in 0..h {
        for x in 0..w {
            
            let mstarty = y - 1;
            let mstartx = x - 1;
            
            let mut accum_red: i32 = 0;
            let mut accum_green: i32 = 0;
            let mut accum_blue: i32 = 0;
            let mut accum_alpha: i32 = 0;

            let mut m_index_y = 0;
            for mut src_y in mstarty..mstarty + m_size {
                if src_y < 0 {
                    src_y = 0;
                } else if src_y > h - 1 {
                    src_y = h - 1;
                }
                let mut m_index_x = 0;
                for mut src_x in mstartx..mstartx + m_size {
                    if src_x < 0 {
                        src_x = 0;
                    } else if src_x > w - 1 {
                        src_x = w - 1;
                    }
                    
                    let pixel = try!(copy.get_pixel(src_x, src_y));
                    accum_red += pixel.r as i32 * matrix[m_index_y][m_index_x];
                    accum_green += pixel.g as i32 * matrix[m_index_y][m_index_x];
                    accum_blue += pixel.b as i32 * matrix[m_index_y][m_index_x];
                    accum_alpha += pixel.a as i32 * matrix[m_index_y][m_index_x];

                    m_index_x+=1;
                }
                m_index_y+=1;
            }

            if divisor != 1 {
                accum_red /= divisor;
                accum_green /= divisor;
                accum_blue /= divisor;
                accum_alpha /= divisor;
            }

            if accum_red < 0 {
                accum_red = 0;
            }
            if accum_green < 0 {
                accum_green = 0;
            }
            if accum_blue < 0 {
                accum_blue = 0;
            }
            if accum_alpha < 0 {
                accum_alpha = 0;
            }

            if accum_red > 255 {
                accum_red = 255;
            }
            if accum_green > 255 {
                accum_green = 255;
            }
            if accum_blue > 255 {
                accum_blue = 255;
            }
            if accum_alpha > 255 {
                accum_alpha = 255;
            }

            try!(src.set_pixel(x, y, Color::rgba(accum_red as u8, accum_green as u8, accum_blue as u8, accum_alpha as u8)));
            
        }
    }
    
    Ok(())
}

/// Apply emboss.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::emboss(&mut image).unwrap();
/// raster::save(&image, "tests/out/test_filter_emboss.jpg");
/// ```
///
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
/// 
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_emboss.jpg)
///
pub fn emboss(mut src: &mut Image) -> Result<(), String>{
    let matrix: [[i32; 3]; 3] = [
        [-2, -1, 0],
        [-1, 1, 1],
        [0, 1, 2]
    ];

    try!(convolve(&mut src, matrix, 1));

    Ok(())
}

/// Apply a gamma correction. 
///
/// Gamma can be a value from 0.01 - 9.99. 
/// A gamma < 1.0 will darken and a gamma > 1.0 will lighten the image.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::gamma(&mut image, 2.0).unwrap();
/// raster::save(&image, "tests/out/test_filter_gamma.jpg");
/// ```
///
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
/// 
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_gamma.jpg)
///
// http://stackoverflow.com/questions/14088889/changing-a-color-brightness
pub fn gamma(mut src: &mut Image, gamma: f32) -> Result<(), String>{
    let w: i32 = src.width;
    let h: i32 = src.height;

    if gamma < 0.01 || gamma > 9.99{
        return Err(format!("Incorrect gamma value {}. Must be in range 0.01 - 9.99.", gamma));
    }
    let gamma = 1.0 / gamma;

    for y in 0..h {
        for x in 0..w {
            
            let p = try!(src.get_pixel(x, y));
            let r = (p.r as f32 / 255.0).powf(gamma) * 255.0;
            let g = (p.g as f32 / 255.0).powf(gamma) * 255.0;
            let b = (p.b as f32 / 255.0).powf(gamma) * 255.0;
            let a = (p.a as f32 / 255.0).powf(gamma) * 255.0;
            
            try!(src.set_pixel(x, y, Color::rgba(r as u8, g as u8, b as u8, a as u8)));
            
        }
    }
    
    Ok(())
}

/// Turn into grayscale image.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::grayscale(&mut image).unwrap();
/// raster::save(&image, "tests/out/test_filter_grayscale.jpg");
/// ```
///
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
/// 
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_grayscale.jpg)
///
pub fn grayscale(mut src: &mut Image) -> Result<(), String>{
    let w: i32 = src.width;
    let h: i32 = src.height;
    
    for y in 0..h {
        for x in 0..w {
            
            let p = try!(src.get_pixel(x, y));
            let gray = (p.r as f32 * 0.3) + (p.g as f32 * 0.59) + (p.b as f32 * 0.11);
            
            try!(src.set_pixel(x, y, Color::rgba(gray as u8, gray as u8, gray as u8, gray as u8)));
            
        }
    }
    
    Ok(())
}

/// Apply sharpen.
///
/// # Examples
/// ```
/// use raster::filter;
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.jpg").unwrap();
/// filter::sharpen(&mut image).unwrap();
/// raster::save(&image, "tests/out/test_filter_sharpen.jpg");
/// ```
/// ### Before
/// ![](https://kosinix.github.io/raster/in/sample.jpg)
/// 
/// ### After
/// ![](https://kosinix.github.io/raster/out/test_filter_sharpen.jpg)
///
pub fn sharpen(mut src: &mut Image) -> Result<(), String>{
    let matrix: [[i32; 3]; 3] = [
        [0, -1, 0],
        [-1, 5,-1],
        [0, -1, 0]
    ];

    try!(convolve(&mut src, matrix, 1));

    Ok(())
}


// Private functions

// Box
fn blur_box(mut src: &mut Image) -> Result<(), String>{
    let matrix: [[i32; 3]; 3] = [
        [1,1,1],
        [1,1,1],
        [1,1,1]
    ];

    try!(convolve(&mut src, matrix, 9));

    Ok(())
}

// Gaussian
fn blur_gaussian(mut src: &mut Image) -> Result<(), String>{
    let matrix: [[i32; 3]; 3] = [
        [1,2,1],
        [2,4,2],
        [1,2,1]
    ];

    try!(convolve(&mut src, matrix, 16));

    Ok(())
}