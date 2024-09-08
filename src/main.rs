use std::{process::exit, thread};
use tic_tac_toe::{args, client::Client, server, spec::Spectator};
use local_ip_address::local_ip;

fn main() {
    let matches = args::new();

    let mode = matches.get_one::<String>("mode").unwrap();
    if mode == "spec" {
        let url = format!(
            "ws://{}:8081",
            matches.get_one::<String>("address").unwrap()
        );
        Spectator::start(&url);
        exit(0);
    }

    let nickname = matches.get_one::<String>("nick").unwrap();

    let ctrl_c_event_message = match mode.as_str() {
        "host" => "Host closed",
        _ => "Guest closed"
    };
    ctrlc::set_handler(move || {
        dbg!(ctrl_c_event_message );
        exit(0);
    }).expect("Error setting Ctrl-C handler");

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
    Client::start(&server_url, Some('✖'), "host", nickname);

    server_thread.join().unwrap();
}