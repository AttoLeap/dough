use std::io::Write;
use std::process::exit;
use std::{fs, io};

use clap::Parser;
use dough::generator::image::ImageGenerator;
use dough::generator::text::TextGenerator;
use dough::generator::Generator;
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod cli;

fn main() {
    let cmd = cli::Command::parse();
    match fs::create_dir_all(cmd.path.clone()) {
        Ok(_) => println!("Created `{}` directory as it didn't exist", cmd.path),
        Err(err) => match err.kind() {
            io::ErrorKind::AlreadyExists => {
                println!("The output directory {} already exists...", cmd.path)
            }
            e => {
                eprintln!("Failed to create output directory: {}", e);
                exit(1)
            }
        },
    };
    match cmd.subcmd {
        cli::SubCommand::Generate(gen_cmd) => match gen_cmd {
            cli::GenerateCmd::Text { size } => {
                let text_gen = TextGenerator::new(size);
                (1..=cmd.count)
                    .into_par_iter()
                    .progress_count(cmd.count)
                    .map(|i| {
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
                    })
                    .collect_vec_list();
            }
            cli::GenerateCmd::Image {
                width,
                height,
                codec,
            } => {
                let text_gen = ImageGenerator::new(width, height, codec);
                (1..=cmd.count)
                    .into_par_iter()
                    .progress_count(cmd.count)
                    .map(|i| {
                        let out: Box<dyn Write> = match (cmd.to_stdout, cmd.to_stderr) {
                            (true, false) => Box::new(io::stdout()),
                            (false, true) => Box::new(io::stderr()),
                            (false, false) => {
                                let file =
                                    format!("{}/file_{}.{}", cmd.path, i, codec.get_extension());
                                Box::new(fs::File::create(file).unwrap())
                            }
                            (true, true) => {
                                eprintln!("Conflicting flags specified");
                                exit(1);
                            }
                        };
                        text_gen.generate(out);
                    })
                    .collect_vec_list();
            }
        },
    }
}
