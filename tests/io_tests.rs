extern crate raster;

#[test]
fn open_fail(){

    let fail = {
        match raster::open(""){
            Ok(_) => false,
            Err(_) => true
        }
    };
    assert!(fail);
}

#[test]
fn unsupported_format(){

    let fail = {
        match raster::open("tests/in/unsupported.txt"){
            Ok(_) => false,
            Err(e) => {
                match e {
                    raster::error::RasterError::UnsupportedFormat(_) => true,
                    _ => false,
                }
            }
        }
    };
    assert!(fail);
}

#[test]
fn read_gif_format(){

    let ok = {
        match raster::open("tests/in/sample.gif"){
            Ok(_) => true,
            Err(_) => false
        }
    };
    assert!(ok);
}

#[test]
fn read_jpg_format(){

    let ok = {
        match raster::open("tests/in/sample.jpg"){
            Ok(_) => true,
            Err(_) => false
        }
    };
    assert!(ok);
}

#[test]
fn read_png_format(){

    let ok = {
        match raster::open("tests/in/sample.png"){
            Ok(_) => true,
            Err(_) => false
        }
    };
    assert!(ok);
}