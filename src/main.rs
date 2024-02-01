use std::{process::exit, thread};
use clap::Parser;
use tic_tac_toe::{host::Host, server, structs::args::Args};

fn main() {
    let args = Args::parse();

    if args.mode == "guest" {
        println!("Just skiping because im a guest.");
        exit(0);
    }

    let server_thread = thread::spawn(move || {
        server::server::start();
    });

    Host::start();

    server_thread.join().unwrap();
}