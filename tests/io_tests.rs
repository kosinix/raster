extern crate raster;

#[test]
fn open_fail(){

    assert!(
        {
            if let Err(_) = raster::open(""){
                true
            } else {
                false
            }
        }
    );
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
        if let Ok(_) = raster::open("tests/in/sample.gif"){
            true
        } else {
            false
        }
    };
    assert!(ok);
}

#[test]
fn read_jpg_format(){

    let ok = {
        if let Ok(_) = raster::open("tests/in/sample.jpg"){
            true
        } else {
            false
        }
    };
    assert!(ok);
}

#[test]
fn read_png_format(){

    let ok = {
        if let Ok(_) = raster::open("tests/in/sample.png"){
            true
        } else {
            false
        }
    };
    assert!(ok);
}

#[test]
fn read_gif_format_fail(){

    assert!( 
        {
            if let Err(e) = raster::open("tests/in/not-a-gif.gif"){
                if let raster::error::RasterError::Decode(format, _) = e {
                    assert!( 
                        {
                            if let raster::ImageFormat::Gif = format { // Should be gif
                                true
                            } else {
                                false
                            }
                        }
                    );
                }
                true
            } else {
                false
            }
        }
    );
}

#[test]
fn read_jpeg_format_fail(){

    assert!( 
        {
            if let Err(e) = raster::open("tests/in/not-a-jpeg.jpg"){
                if let raster::error::RasterError::Decode(format, _) = e {
                    assert!( 
                        {
                            if let raster::ImageFormat::Jpeg = format { // Should be jpg
                                true
                            } else {
                                false
                            }
                        }
                    );
                }
                true
            } else {
                false
            }
        }
    );
}

#[test]
fn read_png_format_fail(){

    assert!( 
        {
            if let Err(e) = raster::open("tests/in/not-a-png.png"){
                if let raster::error::RasterError::Decode(format, _) = e {
                    assert!( 
                        {
                            if let raster::ImageFormat::Png = format { // Should be png
                                true
                            } else {
                                false
                            }
                        }
                    );
                }
                true
            } else {
                false
            }
        }
    );
}