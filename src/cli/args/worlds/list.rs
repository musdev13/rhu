use clap::Args;

#[derive(Args)]
pub struct ListArgs{
    #[arg(long, short)]
    pub raw: bool,
}
