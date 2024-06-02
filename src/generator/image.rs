use clap::ValueEnum;
use image::{
    codecs::{bmp::BmpEncoder, jpeg::JpegEncoder, png::PngEncoder, webp::WebPEncoder},
    ImageBuffer, ImageEncoder, Rgb,
};

use super::Generator;

pub struct ImageGenerator {
    width: u32,
    height: u32,
    codec: ImageCodec,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum ImageCodec {
    BMP,
    JPEG,
    PNG,
    WEBP,
}

impl ImageCodec {
    pub fn get_extension(self) -> &'static str {
        match self {
            ImageCodec::BMP => "bmp",
            ImageCodec::JPEG => "jpeg",
            ImageCodec::PNG => "png",
            ImageCodec::WEBP => "webp",
        }
    }
}

impl ImageGenerator {
    pub fn new(width: u32, height: u32, codec: ImageCodec) -> ImageGenerator {
        return ImageGenerator {
            width,
            height,
            codec,
        };
    }

    fn generate_image_buffer(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut buf = image::ImageBuffer::new(self.width, self.height);
        let r_mul = fastrand::f64();
        let g_mul = fastrand::f64();
        let b_mul = fastrand::f64();
        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            let r = (r_mul * x as f64) as u8;
            let g = (g_mul * x.abs_diff(y) as f64) as u8;
            let b = (b_mul * y as f64) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        buf
    }
}

impl Generator for ImageGenerator {
    fn generate(&self, mut out: impl std::io::Write) {
        let buf = self.generate_image_buffer();
        match self.codec {
            ImageCodec::BMP => {
                BmpEncoder::new(&mut out)
                    .write_image(
                        &buf,
                        self.width,
                        self.height,
                        image::ExtendedColorType::Rgb8,
                    )
                    .unwrap();
            }
            ImageCodec::JPEG => {
                JpegEncoder::new(out)
                    .write_image(
                        &buf,
                        self.width,
                        self.height,
                        image::ExtendedColorType::Rgb8,
                    )
                    .unwrap();
            }
            ImageCodec::PNG => {
                PngEncoder::new(out)
                    .write_image(
                        &buf,
                        self.width,
                        self.height,
                        image::ExtendedColorType::Rgb8,
                    )
                    .unwrap();
            }
            ImageCodec::WEBP => {
                WebPEncoder::new_lossless(out)
                    .write_image(
                        &buf,
                        self.width,
                        self.height,
                        image::ExtendedColorType::Rgb8,
                    )
                    .unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use image::{codecs::png::PngDecoder, ImageDecoder};

    use crate::generator::{image::ImageCodec, Generator};

    use super::ImageGenerator;

    #[test]
    fn test_image_size() {
        let width = 1000;
        let height = 900;
        let result_size = (width * height * 3) as usize;
        let img_gen = ImageGenerator::new(width, height, ImageCodec::PNG);
        let buf = img_gen.generate_image_buffer();
        assert_eq!(buf.len(), result_size);
    }

    #[test]
    fn test_png_encoder() {
        let width = 900;
        let height = 500;
        let result_size = (width * height * 3) as u64;
        let img_gen = ImageGenerator::new(width, height, ImageCodec::PNG);
        let mut buf = Vec::new();
        img_gen.generate(&mut buf);
        let decoder = PngDecoder::new(Cursor::new(buf)).unwrap();
        assert_eq!(decoder.total_bytes(), result_size);
        assert_eq!(decoder.dimensions().0, width);
        assert_eq!(decoder.dimensions().1, height);
    }
}
