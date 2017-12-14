extern crate raster;

use raster::Color;

#[test]
fn hsb_test() {
    let hsv = Color::to_hsv(50, 50, 100);

    assert_eq!(240, hsv.0);
    assert_eq!(50, (hsv.1).round() as i32); // round and cast to integer because float
    assert_eq!(39, (hsv.2).round() as i32);
}

#[test]
fn conversion_accuracy_test() {
    let rgb1 = (127, 70, 60);
    let hsv = Color::to_hsv(rgb1.0, rgb1.1, rgb1.2);
    let rgb2 = Color::to_rgb(hsv.0, hsv.1, hsv.2);

    assert_eq!(rgb1.0, rgb2.0);
    assert_eq!(rgb1.1, rgb2.1);
    assert_eq!(rgb1.2, rgb2.2);
}

#[test]
fn hex_test() {
    // Ok tests
    let color = Color::hex("#FFFFFF"); // Opaque white
    assert!(color.is_ok());

    let color = Color::hex("#00FF007F"); // Green with 50% opacity
    assert!(color.is_ok());

    // Error tests
    let color = Color::hex("");
    assert!(color.is_err());

    let color = Color::hex("#");
    assert!(color.is_err());

    let color = Color::hex("#FFF");
    assert!(color.is_err());
}
