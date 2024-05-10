use super::Generator;

pub struct ImageGenerator {
    width: u32,
    height: u32,
}

impl ImageGenerator {
    pub fn new(width: u32, height: u32) -> ImageGenerator {
        return ImageGenerator {width, height}
    }
}

impl Generator for ImageGenerator {
    fn generate(&self, mut out: impl std::io::Write) {
        let mut buf = image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            let r = (0.4 * x as f64) as u8;
            let b = (0.6 * y as f64) as u8;
            *pixel = image::Rgb([r, 0, b]);
        }
        out.write(&buf.to_vec()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::Generator;

    use super::ImageGenerator;


    #[test]
    fn test_image_size() {
        let width = 1000;
        let height = 900;
        let result_size = (width * height * 3) as usize;
        let img_gen = ImageGenerator::new(width, height);
        let mut buf = Vec::new();
        img_gen.generate(&mut buf);
        assert_eq!(buf.len(), result_size);
    }

}