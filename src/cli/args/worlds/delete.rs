use clap::Args;

#[derive(Args)]
pub struct DeleteArgs{
    #[arg(long, short)]
    pub raw: bool,
    pub save_name: String,
}
