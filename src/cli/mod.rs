use dough::generator::image::ImageCodec;

#[derive(clap::Parser)]
#[clap(version, about)]
pub struct Command {
    #[command(subcommand)]
    pub subcmd: SubCommand,
    #[clap(long, short, global = true, default_value = ".")]
    pub path: String,
    #[clap(long, global = true, default_value_t = false)]
    pub to_stdout: bool,
    #[clap(long, global = true, default_value_t = false)]
    pub to_stderr: bool,
    #[clap(long, short, global = true, default_value_t = 1)]
    pub count: u64,
}

#[derive(clap::Subcommand)]
pub enum SubCommand {
    #[command(subcommand, aliases=["gen"])]
    Generate(GenerateCmd),
}

#[derive(clap::Subcommand)]
pub enum GenerateCmd {
    Text {
        #[clap(long, short, default_value_t = 1024)]
        size: u128,
    },
    Image {
        #[clap(long, short, default_value_t = 1024)]
        width: u32,
        #[clap(long, short = 'H', default_value_t = 1024)]
        height: u32,
        #[clap(long, short = 'C', default_value = "png")]
        codec: ImageCodec,
    },
}
