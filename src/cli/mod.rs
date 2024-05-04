#[derive(clap::Parser)]
#[clap(version,about)]
pub struct Command {
    #[command(subcommand)]
    pub subcmd: SubCommand
}

#[derive(clap::Subcommand)]
pub enum SubCommand {
    #[command(subcommand, aliases=["gen"])]
    Generate(GenerateCmd)
}

#[derive(clap::Subcommand)]
pub enum GenerateCmd {
    Text {
        #[clap(long,short,default_value = ".")]
        path: String,
        #[clap(long, default_value_t = false)]
        to_stdout: bool,
        #[clap(long, default_value_t = false)]
        to_stderr: bool,
        #[clap(long,short,default_value_t = 1024)]
        size: u128
    }
}