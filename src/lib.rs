//! Rust bindings for TurboJPEG, which provides simple and fast compression/decompression of JPEG
//! images.
//!
//! # High-level usage with image-rs
//! 
//! To easily encode and decode images from the [`image`][image-rs] crate, please
//! enable the optional dependency by adding this to the `[dependencies]` section of
//! your `Cargo.toml`:
//! 
//! ```toml
//! turbojpeg = {version = "^0.2", features = ["image"]}
//! ```
//! 
//! Then you can use the functions [`decompress_image`] and
//! [`compress_image`] to easily decode and encode JPEG:
//! 
//! ```rust
//! // create an `image::RgbImage`
//! let image: image::RgbImage = ...;
//! // compress `image` into JPEG with quality 95 and no chrominance subsampling
//! let jpeg_data = turbojpeg::compress_image(&image, 95, turbojpeg::Subsamp::None)?;
//! 
//! // decompress `jpeg_data` into an `image::RgbImage`
//! let image: image::RgbImage = turbojpeg::decompress_image(&jpeg_data);
//! ```
//! 
//! This crate supports these image types:
//! 
//! - [`RgbImage`][::image::RgbImage]
//! - [`RgbaImage`][::image::RgbaImage] (JPEG does not support alpha channel, so alpha is ignored
//!   when encoding and set to 255 when decoding)
//! - [`GrayImage`][::image::GrayImage]
//! 
//! [image-rs]: https://docs.rs/image/*/image/index.html
//! 
//! # Low-level usage with `Compressor`/`Decompressor`
//! 
//! Use [`Compressor`] to compress raw pixel data into JPEG
//! (see [`examples/compressor.rs`][compressor-example] for full example):
//!
//! [compressor-example]: https://github.com/honzasp/rust-turbojpeg/blob/master/examples/compressor.rs
//! 
//! ```rust
//! use turbojpeg::{Compressor, Image, PixelFormat};
//! 
//! // prepare the raw pixel data
//! let width: usize = ...;
//! let height: usize = ...;
//! let pixels: Vec<u8> = ...;
//! 
//! // initialize a Compressor
//! let mut compressor = Compressor::new()?;
//! 
//! // create an Image that bundles a reference to the raw pixel data (as &[u8])
//! // with information about the image format
//! let image = Image {
//!     // &[u8] reference to the pixel data
//!     pixels: pixels.as_slice(),
//!     // width of the image in pixels
//!     width: width,
//!     // size of the image row in bytes (also called "stride")
//!     pitch: 3 * width,
//!     // height of the image in pixels
//!     height: height,
//!     // format of the pixel data
//!     format: PixelFormat::RGB,
//! };
//!
//! // compress the Image to a Vec<u8> of JPEG data
//! let jpeg_data = compressor.compress_to_vec(image)?;
//! ```
//! 
//! To decompress JPEG data into a raw pixel data, use [`Decompressor`] (full example in
//! [`examples/decompressor.rs`][decompressor-example]):
//!
//! [decompressor-example]: https://github.com/honzasp/rust-turbojpeg/blob/master/examples/decompressor.rs
//! 
//! ```rust
//! use turbojpeg::{Decompressor, Image, PixelFormat};
//! 
//! // get the JPEG data
//! let jpeg_data: &[u8] = ...;
//! 
//! // initialize a Decompressor
//! let mut decompressor = Decompressor::new()?;
//! 
//! // read the JPEG header with image size
//! let header = decompressor.read_header(jpeg_data)?;
//! let (width, height) = (header.width, header.height);
//! 
//! // prepare a storage for the raw pixel data
//! let mut pixels = vec![0; 3*width*height];
//! let image = Image {
//!     // &mut [u8] reference to the image data
//!     pixels: pixels.as_mut_slice(),
//!     width: width,
//!     pitch: 3 * width,
//!     height: height,
//!     format: PixelFormat::RGB,
//! };
//! 
//! // decompress the JPEG data 
//! decompressor.decompress_to_slice(jpeg_data, image)?;
//! 
//! // use the raw pixel data
//! println!("{:?}", &pixels[0..9]);
//! ```
//! 
//! # Features
//!
//! - `image`: enables the optional dependency on the [`image`][image-rs] crate.
//! - `pkg-config`: uses pkg-config to find the `libturbojpeg` library.
//! - `bindgen`: uses [bindgen] to generate the `libturbojpeg` bindings.
//!
//! [bindgen]: https://rust-lang.github.io/rust-bindgen/
#![warn(missing_docs)]

pub extern crate turbojpeg_sys as raw;
pub extern crate libc;

mod buf;
mod common;
mod compress;
mod decompress;
mod image;
mod transform;
pub use buf::{OwnedBuf, OutputBuf};
pub use common::{PixelFormat, Subsamp, Colorspace, Result, Error, compressed_buf_len};
pub use compress::Compressor;
pub use decompress::{Decompressor, DecompressHeader};
pub use image::Image;
pub use transform::{Transformer, Transform, TransformOp, TransformCrop};

#[cfg(feature = "image")]
mod image_rs;
#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
pub use self::image_rs::{JpegPixel, compress_image, decompress_image};

