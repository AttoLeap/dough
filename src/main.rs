use std::io::Write;
use std::process::exit;
use std::{fs, io};

use clap::Parser;
use dough::generator::text::TextGenerator;
use dough::generator::Generator;

mod cli;

fn main() {
    let cmd = cli::Command::parse();
    match cmd.subcmd {
        cli::SubCommand::Generate(gen_cmd) => match gen_cmd {
            cli::GenerateCmd::Text {
                path,
                size,
                to_stdout,
                to_stderr,
            } => {
                let text_gen = TextGenerator::new(size);
                let out: Box<dyn Write> = match (to_stdout, to_stderr) {
                    (true, false) => Box::new(io::stdout()),
                    (false, true) => Box::new(io::stderr()),
                    (false, false) => {
                        let file = format!("{}/file.txt", path);
                        Box::new(fs::File::create(file).unwrap())
                    }
                    (true, true) => {
                        eprintln!("Conflicting flags specified");
                        exit(1);
                    }
                };
                text_gen.generate(out);
            }
        },
    }
}
