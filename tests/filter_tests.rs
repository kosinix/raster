extern crate raster;

use raster::filter;

#[test]
fn brightness_test(){

    let mut image = raster::open("tests/in/sample.jpg").unwrap();
    filter::brightness(&mut image, 1.5).unwrap();
    raster::save(&image, "tests/out/test_filter_brightness.jpg").unwrap();
}
