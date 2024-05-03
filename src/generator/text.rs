use super::Generator;
use std::io;

pub struct TextGenerator {
    char_count: u128,
}

impl TextGenerator {
    pub fn new(char_count: u128) -> TextGenerator {
        return TextGenerator { char_count };
    }
}

impl Generator for TextGenerator {
    fn generate(&self, mut out: impl io::Write) {
        for _ in 0..self.char_count {
            out.write(fastrand::alphanumeric().to_string().as_bytes())
                .unwrap();
        }
    }
}
