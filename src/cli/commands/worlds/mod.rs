use clap::Subcommand;

use crate::cli::args::worlds::{delete::DeleteArgs, list::ListArgs};
pub mod list;
pub mod delete;

#[derive(Subcommand)]
pub enum Commands{
    #[command(about = "Worlds list")]
    List(ListArgs),
    Import,
    Export,
    #[command(about = "Delete world")]
    Delete(DeleteArgs),
}
