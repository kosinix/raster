extern crate raster;

use raster::Color;

fn main(){

    let hsv = Color::to_hsv(50,50,100);
    
    println!("{:?}", (hsv.2).round());

}