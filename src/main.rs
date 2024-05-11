use std::io::Write;
use std::process::exit;
use std::{fs, io};

use clap::Parser;
use dough::generator::image::ImageGenerator;
use dough::generator::text::TextGenerator;
use dough::generator::Generator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod cli;

fn main() {
    let cmd = cli::Command::parse();
    match cmd.subcmd {
        cli::SubCommand::Generate(gen_cmd) => match gen_cmd {
            cli::GenerateCmd::Text {
                size,
            } => {
                let text_gen = TextGenerator::new(size);
                (1..=cmd.count).into_par_iter().map(|i| {
                    let out: Box<dyn Write> = match (cmd.to_stdout, cmd.to_stderr) {
                        (true, false) => Box::new(io::stdout()),
                        (false, true) => Box::new(io::stderr()),
                        (false, false) => {
                            let file = format!("{}/file_{}.txt", cmd.path, i);
                            Box::new(fs::File::create(file).unwrap())
                        }
                        (true, true) => {
                            eprintln!("Conflicting flags specified");
                            exit(1);
                        }
                    };
                    text_gen.generate(out);
                }).collect_vec_list();
            }
            cli::GenerateCmd::Image {
                width,
                height,
                codec,
            } => {
                let text_gen = ImageGenerator::new(width, height, codec);
                (1..=cmd.count).into_par_iter()
                    .map(|i| {
                        let out: Box<dyn Write> = match (cmd.to_stdout, cmd.to_stderr) {
                            (true, false) => Box::new(io::stdout()),
                            (false, true) => Box::new(io::stderr()),
                            (false, false) => {
                                let file = format!("{}/file_{}.{}", cmd.path, i, codec.get_extension());
                                Box::new(fs::File::create(file).unwrap())
                            }
                            (true, true) => {
                                eprintln!("Conflicting flags specified");
                                exit(1);
                            }
                        };
                        text_gen.generate(out);
                    }).collect_vec_list();
            }
        },
    }
}
