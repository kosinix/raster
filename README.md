# Raster

An image processing library for Rust

## Documentation

Examples and API are at [https://docs.rs/raster/](https://docs.rs/raster/)


## Q&A

* **What is this?** - An image processing library in pure Rust. It should allow you to easily modify the pixels of raster images such as JPEG, PNG and GIF.
* **Why?** - Why not? Its born out of my desire to learn Rust. Another image processing lib wouldn't hurt the community.
* **Why not piston image?** - My personal opinion is its difficult to use. A simplified alternative would be handy.
* **But it does depend on piston image?** Yes for opening and saving an image it uses piston image. Everything else is done by raster. Eventually raster will change its dependency to only use the bare image decoders/encoders.
* **Whats the goal?** - To add more and more features until it becomes an advanced image processing lib. At the same time, it should keep everything simple.
* **Whats the possible use-case?** - I can see it being used in web apps. Maybe other people would find it useful. We'll see.

## License
- MIT License