use std::{process::exit, thread};
use clap::{builder::PossibleValue, Arg, Command};
use tic_tac_toe::{guest::Guest, host::Host, server};
use local_ip_address::local_ip;

fn main() {
    let matches = Command::new("Tic Tac Toe")
        .arg(
            Arg::new("nick")
            .value_name("NICK")
            .short('n')
            .long("nick")
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
            .required_if_eq("mode", "guest")
        ).get_matches();

    let mode = matches.get_one::<String>("mode").unwrap();
    let nickname = matches.get_one::<String>("nick").unwrap();

    if mode == "guest" {
        let url = format!(
            "ws://{}:8081",
            matches.get_one::<String>("address").unwrap()
        );
        Guest::start(&url);
        exit(0);
    }

    let server_thread = thread::spawn(move || {
        server::server::start();
    });

    let server_url = format!("ws://{}:8081", local_ip().unwrap());
    Host::start(&server_url, nickname);

    server_thread.join().unwrap();
}