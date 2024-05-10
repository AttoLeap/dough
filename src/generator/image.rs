

use image::{codecs::png::PngEncoder, ImageBuffer, ImageEncoder, Rgb};

use super::Generator;

pub struct ImageGenerator {
    width: u32,
    height: u32,
}

impl ImageGenerator {
    pub fn new(width: u32, height: u32) -> ImageGenerator {
        return ImageGenerator { width, height };
    }

    fn generate_image_buffer(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut buf = image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            let r = (0.3 * x as f64) as u8;
            let g = (0.4 * x.abs_diff(y) as f64) as u8;
            let b = (0.5 * y as f64) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        buf
    }
}

impl Generator for ImageGenerator {
    fn generate(&self, out: impl std::io::Write) {
        let buf = self.generate_image_buffer();
        let encoder = PngEncoder::new(out);
        encoder
            .write_image(
                &buf.to_vec(),
                self.width,
                self.height,
                image::ExtendedColorType::Rgb8,
            )
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use image::{codecs::png::PngDecoder, ImageDecoder};

    use crate::generator::Generator;

    use super::ImageGenerator;

    #[test]
    fn test_image_size() {
        let width = 1000;
        let height = 900;
        let result_size = (width * height * 3) as usize;
        let img_gen = ImageGenerator::new(width, height);
        let buf = img_gen.generate_image_buffer();
        assert_eq!(buf.len(), result_size);
    }

    #[test]
    fn test_image_encoder() {
        let width = 900;
        let height = 500;
        let result_size = (width * height * 3) as u64;
        let img_gen = ImageGenerator::new(width, height);
        let mut buf = Vec::new();
        img_gen.generate(&mut buf);
        let decoder = PngDecoder::new(Cursor::new(buf)).unwrap();
        assert_eq!(decoder.total_bytes(), result_size);
        assert_eq!(decoder.dimensions().0, width);
        assert_eq!(decoder.dimensions().1, height);
    }
}
