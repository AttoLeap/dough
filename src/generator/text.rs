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

const BUF_SIZE: u16 = 256;

impl Generator for TextGenerator {
    fn generate(&self, mut out: impl io::Write) {
        let mut length = self.char_count;
        while length >= BUF_SIZE as u128 {
            length -= BUF_SIZE as u128;
            let string = generate_string(BUF_SIZE);
            out.write(string.as_bytes()).unwrap();
        }
        if length > 0 {
            out.write(generate_string(length as u16).as_bytes())
                .unwrap();
        }
    }
}

fn generate_string(length: u16) -> String {
    let mut string: String = "".to_string();
    for _ in 0..length {
        string.push(fastrand::alphanumeric());
    }
    return string;
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::generate_string;

    #[test]
    fn test_string_generator() {
        let length = 999;
        let result = generate_string(length);
        let re = Regex::new("[a-zA-Z0-9]").unwrap();
        assert_eq!(result.len(), length as usize);
        assert!(re.is_match(&result));
    }
}
