use clap::Subcommand;
pub mod routes;
pub mod worlds;

#[derive(Subcommand)]
pub enum Commands {
   #[command(about = "Worlds/Universes manage")]
   Worlds {
    #[command(subcommand)]
    command: worlds::Commands,
   }
}
