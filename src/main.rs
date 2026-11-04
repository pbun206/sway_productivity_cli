use clap::Parser;
use clap::Subcommand;
mod rename;
mod swap_indicies;
mod switch;
mod utils;

use anyhow::Result;
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    SwitchTo { index: i32 },
    Rename,
    SwapWith { index: i32 },
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        SubCommands::SwitchTo { index } => switch::switch_to(index),
        SubCommands::Rename => rename::rename(),
        SubCommands::SwapWith { index } => swap_indicies::swap_with(index),
    }
}
