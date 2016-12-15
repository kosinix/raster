//!  A module for 2D transformation.

// crates


// from rust
use std::cmp;

// from external crate


// from local crate
use Image;
use Color;

/// Rotate an image clockwise. Negate the degrees to do a counter-clockwise rotation. Background color can be any color.
///
/// # Examples
/// ```
/// use raster::transform;
/// use raster::Color;
///
/// // Create image from file
/// let mut image = raster::open("tests/in/sample.png").unwrap();
/// transform::rotate(&mut image, 45, Color::rgb(0,0,0)).unwrap();
/// raster::save(&image, "tests/out/test_transform_rotate_45.png");
///
/// let mut image = raster::open("tests/in/sample.png").unwrap();
/// transform::rotate(&mut image, -45, Color::rgb(255,0,0)).unwrap();
/// raster::save(&image, "tests/out/test_transform_rotate_45cc.png");
/// ```
pub fn rotate(mut src: &mut Image, degree: i32, bg: Color) -> Result<&mut Image, String>{
    
    let w1 = src.width;
    let h1 = src.height;

    let degree = degree as f32; // convert to float

    // Using screen coords system, top left is always at (0,0)
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let top_right_1: (i32, i32) = (w1, 0);
    let top_right_2: (i32, i32) = _rotate(top_right_1, degree);
    min_x = cmp::min(min_x, top_right_2.0);
    max_x = cmp::max(max_x, top_right_2.0);
    min_y = cmp::min(min_y, top_right_2.1);
    max_y = cmp::max(max_y, top_right_2.1);

    let bottom_right_1: (i32, i32) = (w1, h1);
    let bottom_right_2: (i32, i32) = _rotate(bottom_right_1, degree);
    min_x = cmp::min(min_x, bottom_right_2.0);
    max_x = cmp::max(max_x, bottom_right_2.0);
    min_y = cmp::min(min_y, bottom_right_2.1);
    max_y = cmp::max(max_y, bottom_right_2.1);

    let bottom_left_1: (i32, i32) = (0, h1);
    let bottom_left_2: (i32, i32) = _rotate(bottom_left_1, degree);
    min_x = cmp::min(min_x, bottom_left_2.0);
    max_x = cmp::max(max_x, bottom_left_2.0);
    min_y = cmp::min(min_y, bottom_left_2.1);
    max_y = cmp::max(max_y, bottom_left_2.1);

    let w2 = ((min_x as f32).abs() + (max_x as f32).abs()) as i32 + 1;
    let h2 = ((min_y as f32).abs() + (max_y as f32).abs()) as i32 + 1;
    let mut dest = Image::blank(w2, h2);

    let mut dest_y = 0;
    for y in min_y..max_y+1 {

        let mut dest_x = 0;
        for x in min_x..max_x+1{
            let point: (i32, i32) = _rotate((x,y), -degree);

            if point.0 >= 0 && point.0 < w1 && point.1 >=0 && point.1 < h1 {
                let pixel = try!(src.get_pixel(point.0, point.1));
                try!(dest.set_pixel(dest_x, dest_y, pixel));
            } else {
                try!(dest.set_pixel(dest_x, dest_y, Color::rgba(bg.r, bg.g, bg.b, bg.a)));
            }
            dest_x += 1;

        }
        dest_y += 1;
    }

    src.width = dest.width;
    src.height = dest.height;
    src.bytes = dest.bytes;
    
    Ok(src)
}


fn _rotate(p: (i32, i32), deg: f32) -> (i32, i32) {
    let radians:f32 = deg.to_radians();
    let px: f32 = p.0 as f32;
    let py: f32 = p.1 as f32;
    let x = ((px * radians.cos()) - (py * radians.sin())).round();
    let y = ((px * radians.sin()) + (py * radians.cos())).round();
    (x as i32, y as i32)
}
