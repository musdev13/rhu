use clap::Parser;
pub mod commands;
pub mod args;

#[derive(Parser)]
#[command(name="rhu")]
#[command(about="Rust Hytale Utility")]
pub struct Cli{
    #[command(subcommand)]
    pub command: commands::Commands
}

