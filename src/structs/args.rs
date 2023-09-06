use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short='t', long="host-type")]
    pub host_type: String
}