use std::io;

use dough::generator::Generator;
use dough::generator::text::TextGenerator;

fn main() {
    let text_gen = TextGenerator::new(256);
    for i in 0..5 {
        print!("Run {}:\n", i);
        text_gen.generate(io::stdout());
        println!();
    }
}
