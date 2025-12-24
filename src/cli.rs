use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub daemon: bool,

    #[arg(long)]
    pub check: bool,
}
