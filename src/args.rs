use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "get CY weather")]
pub struct Args {
    #[arg(long)]
    pub station: Option<String>,
    #[arg(long)]
    pub stations: bool,
    #[arg(long)]
    pub dry_run: Option<String>,
}
