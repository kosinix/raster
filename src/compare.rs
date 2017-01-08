//!  A module for comparing images.


// from rust

// from external crate

// from local crate
use error::RasterResult;
use Image;
use editor::{self, ResizeMode};

/// Compare two images and returns a hamming distance. A value of 0 indicates a likely similar picture.
/// A value between 1 and 10 is potentially a variation. A value greater than 10 is likely a different image.
///
/// # Examples
/// ```
/// use raster::compare;
///
/// let image1 = raster::open("tests/in/sample.jpg").unwrap();
/// let image2 = raster::open("tests/in/sample.png").unwrap();
///
/// let hamming_distance = compare::similar(&image1, &image2).unwrap();
/// println!("{}", hamming_distance);
/// ```
pub fn similar(image1: &Image, image2: &Image) -> RasterResult<u8> {

    let bin1 = try!(diff_hash(image1));
    let bin2 = try!(diff_hash(image2));
    let mut distance = 0;
    for (index, value) in bin1.iter().enumerate() {
        if value != &bin2[index] {
            distance += 1;
        }
    }
    Ok(distance)
}

/// Compare if two images are equal. It will compare if the two images are of the same width and height.
/// If the dimensions differ, it will return false. If the dimensions are equal, it will loop through each pixels.
/// If one of the pixel don't match, it will return false. The pixels are compared using their RGB (Red, Green, Blue) values.
///
/// # Examples
/// ```
/// use raster::compare;
///
/// let image1 = raster::open("tests/in/sample.png").unwrap();
/// let image2 = raster::open("tests/in/sample.png").unwrap();
///
/// let equal = compare::equal(&image1, &image2).unwrap();
/// assert_eq!(true, equal);
/// ```
pub fn equal(image1: &Image, image2: &Image)-> RasterResult<bool> {

    // Check if image dimensions are equal
    if image1.width != image2.width || image1.height != image2.height {

        Ok(false)
    } else {

        // Loop using image1
        for y in 0..image1.height {
            for x in 0..image1.width {

                // Get image1 pixel
                let pixel1 = try!(image1.get_pixel(x, y));

                // Get image2 pixel
                let pixel2 = try!(image2.get_pixel(x, y));

                // Compare pixel value
                if
                    pixel1.r != pixel2.r ||
                    pixel1.g != pixel2.g ||
                    pixel1.b != pixel2.b
                {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
}

// Private functions

// DifferenceHash
//
// Algorithm:
// Reduce size. The fastest way to remove high frequencies and detail is to shrink the image. In this case, shrink it to 9x8 so that there are 72 total pixels.
// Reduce color. Convert the image to a grayscale picture. This changes the hash from 72 pixels to a total of 72 colors.
// Compute the difference. The algorithm works on the difference between adjacent pixels. This identifies the relative gradient direction. In this case, the 9 pixels per row yields 8 differences between adjacent pixels. Eight rows of eight differences becomes 64 bits.
// Assign bits. Each bit is simply set based on whether the left pixel is brighter than the right pixel.
//
// http://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html
//
//
fn diff_hash(image: &Image) -> RasterResult<Vec<u8>> {

    let width  = 9;
    let height = 8;

    let mut image = image.clone(); // copy it since resize is desctructive
    try!(editor::resize(&mut image, width, height, ResizeMode::Exact)); // Resize to exactly 9x8

    // Build hash
    let mut hash = Vec::new();
    for y in 0..height {

        // Get the pixel value for the leftmost pixel.
        let pixel = try!(image.get_pixel(0, y));
        let mut left = ((pixel.r as f32 + pixel.g as f32 + pixel.b as f32) / 3.0).floor();

        // Get the pixel value for each pixel starting from position 1.
        for x in 1..width {

            let pixel = try!(image.get_pixel(x, y));
            let right = ((pixel.r as f32 + pixel.g as f32 + pixel.b as f32) / 3.0).floor();
            // Each hash bit is set based on whether the left pixel is brighter than the right pixel.
            if left > right {
                hash.push(1);
            } else {
                hash.push(0);
            }
            // Prepare the next loop.
            left = right;
        }
    }
    Ok(hash)
}
