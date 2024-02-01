use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short='m', long="mode")]
    pub mode: String
}