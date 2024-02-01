use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short='m',long="mode")]
    pub mode: String,
    #[clap(short='r',long="repo_url")]
    pub repository_url: String
}