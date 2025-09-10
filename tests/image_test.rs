extern crate raster;

use raster::Image;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_pixel() {
        let mut img = Image::blank(10, 10);

        // Shrink bytes manually to simulate corruption
        img.bytes.truncate(50);
        // Only pixels that fit within 50 bytes are valid
    assert!(img.check_pixel(0, 0)); // start = 0..4
    assert!(img.check_pixel(1, 1)); // start = 44..48
    assert!(!img.check_pixel(2, 2)); // start = 88..92 → exceeds 50
    assert!(!img.check_pixel(3, 3)); // start = 156..160 → exceeds 50
    }
}
