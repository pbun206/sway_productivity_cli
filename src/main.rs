use clap::Parser;
use clap::Subcommand;
mod appstate;
mod rename;
mod splash;
mod swap_indicies;
mod switch;
mod utils;
pub mod window;

use anyhow::Result;
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    SwitchTo { index: u32 },
    Rename,
    SwapWith { index: u32 },
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    match args.cmd {
        SubCommands::SwitchTo { index } => switch::switch_to(index),
        SubCommands::Rename => rename::rename(),
        SubCommands::SwapWith { index } => swap_indicies::swap_with(index),
    }
}
