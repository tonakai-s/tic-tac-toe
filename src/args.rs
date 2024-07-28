use clap::{builder::PossibleValue, Arg, ArgMatches, Command};

pub fn new() -> ArgMatches {
    Command::new("Tic Tac Toe")
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
            ).get_matches()
}