use std::io;

use dough::generator::text::TextGenerator;
use dough::generator::Generator;

fn main() {
    let text_gen = TextGenerator::new(1024);
    for i in 0..5 {
        print!("Run {}:\n", i);
        text_gen.generate(io::stdout());
        println!();
    }
}
