use std::{process::exit, thread};
use tic_tac_toe::{client::Client, server, args};
use local_ip_address::local_ip;

fn main() {
    let matches = args::new();

    let mode = matches.get_one::<String>("mode").unwrap();
    let nickname = matches.get_one::<String>("nick").unwrap();

    if mode == "guest" {
        let url = format!(
            "ws://{}:8081",
            matches.get_one::<String>("address").unwrap()
        );
        Client::start(&url, Some('⬤'), "guest", nickname);
        exit(0);
    }

    let server_thread = thread::spawn(move || {
        server::start();
    });

    let local_addr = local_ip().unwrap();
    let server_url = format!("ws://{}:8081", local_addr);
    // TODO: Sometimes this connection failes, why???
    Client::start(&server_url, Some('✖'), "host", nickname);

    server_thread.join().unwrap();
}