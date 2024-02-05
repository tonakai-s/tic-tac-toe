use std::{process::exit, thread};
use clap::{builder::PossibleValue, Arg, Command};
use tic_tac_toe::{client::Client, server};
use local_ip_address::local_ip;

fn main() {
    let matches = Command::new("Tic Tac Toe")
        .arg(
            Arg::new("nick")
            .value_name("NICK")
            .short('n')
            .long("nick")
            .help("Defines your user on the game.")
            .required(true)
        )
        .arg(
            Arg::new("mode")
            .value_name("MODE")
            .short('m')
            .long("mode")
            .value_parser([
                PossibleValue::new("host").help("Set the client as a host and run the server."),
                PossibleValue::new("guest").help("Set the client as guest and need URL to connect to the server.")
            ])
            .default_value("host")
        )
        .arg(
            Arg::new("address")
            .value_name("ADDRESS")
            .short('a')
            .long("addr")
            .help("Set the server host to connect.")
            .required_if_eq("mode", "guest")
        ).get_matches();

    let mode = matches.get_one::<String>("mode").unwrap();
    let nickname = matches.get_one::<String>("nick").unwrap();

    if mode == "guest" {
        let url = format!(
            "ws://{}:8081",
            matches.get_one::<String>("address").unwrap()
        );
        Client::start(&url, Some('⬤'), "guest", &nickname);
        exit(0);
    }

    let server_thread = thread::spawn(move || {
        server::server::start();
    });

    let server_url = format!("ws://{}:8081", local_ip().unwrap());
    Client::start(&server_url, Some('✖'), "host", nickname);

    server_thread.join().unwrap();
}