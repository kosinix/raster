extern crate raster;

use raster::{filter, Orientation};

#[test]
fn brightness_test(){

    let mut image = raster::open("tests/in/sample.jpg").unwrap();
    filter::brightness(&mut image, 1.5).unwrap();
    raster::save(&image, "tests/out/test_filter_brightness.jpg").unwrap();
}

#[test]
fn sobel_test() {
    let mut image = raster::open("tests/in/sample.jpg").unwrap();
    filter::sobel(&mut image, Orientation::Both).unwrap();
    raster::save(&image, "tests/out/test_filter_sobel.jpg").unwrap();
}

#[test]
fn sobel_x_test() {
    let mut image = raster::open("tests/in/sample.jpg").unwrap();
    filter::sobel(&mut image, Orientation::Horizontal).unwrap();
    raster::save(&image, "tests/out/test_filter_sobel_x.jpg").unwrap();
}

#[test]
fn sobel_y_test() {
    let mut image = raster::open("tests/in/sample.jpg").unwrap();
    filter::sobel(&mut image, Orientation::Vertical).unwrap();
    raster::save(&image, "tests/out/test_filter_sobel_y.jpg").unwrap();
}

#[test]
fn sobel_d1_test() {
    let mut image = raster::open("tests/in/sample.jpg").unwrap();
    filter::sobel(&mut image, Orientation::DiagonalUp).unwrap();
    raster::save(&image, "tests/out/test_filter_sobel_d1.jpg").unwrap();
}

#[test]
fn sobel_d2_test() {
    let mut image = raster::open("tests/in/sample.jpg").unwrap();
    filter::sobel(&mut image, Orientation::DiagonalDown).unwrap();
    raster::save(&image, "tests/out/test_filter_sobel_d2.jpg").unwrap();
}