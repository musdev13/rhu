mod cli;
mod core;

use clap::Parser;


fn main() {
   let cli_handler = cli::Cli::parse();
   cli::commands::routes::routes(cli_handler.command);
}
