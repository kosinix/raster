# Raster

[![Build Status](https://travis-ci.org/kosinix/raster.svg?branch=master)](https://travis-ci.org/kosinix/raster)

An image processing library for Rust. 

## Documentation

API and detailed documentation can be found [here](https://docs.rs/raster/)

## Core Principles
* Keep everything simple
* Stick to primitive or simple types if possible, use advance types when neccessary
* All functions THAT CAN return an error should return the Result type, otherwise return an appropriate type
* Stick to module + functions if possible, no unneccessary abstraction (eg. OOP)
* Function names should be short as they map to the raster-cli commands 
* High speed, low mem

## License
- MIT License

## To Do List

* ~~More examples and add pictures in docs~~
* Implement bicubic interpolation
* ~~Add geometric transformations~~
* ~~Add filters and convolution~~
* Memory and perf improvements
* Reduce dependency to image decoders/encoders