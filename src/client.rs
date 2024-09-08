use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use ws::{connect, Handler, Handshake, Message, Result, Sender};

#[derive(Serialize, Deserialize)]
pub enum ServerState {
    Error { player: String, message: String },
    Info { message: String },
    New { curr_player: String, board: String},
    Winner { player: String, board: String },
    Draw { message: String, board: String }
}

#[derive(Serialize, Deserialize)]
pub enum ClientState<'a> {
    Join { mode: &'a str, nickname: &'a str },
    Play { nickname: &'a str, position: u8, symbol: char }
}

pub struct Client {
    server: Sender,
    symbol: Option<char>,
    mode: String,
    nickname: String
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let join = ClientState::Join { mode: &self.mode, nickname: &self.nickname };
        
        self.server.send(
            Message::text(
            serde_json::to_string(&join).expect("Unable to mount the join message.")
            )
        ).unwrap();

        Ok(())
    }

    fn on_close(&mut self, _: ws::CloseCode, reason: &str) {
        println!("(⌐■_■) Your connection has been closed by the server because of: {}", reason);
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let msg = msg.to_string();
        let state = serde_json::from_str::<ServerState>(&msg).expect(&format!("Received JSON: {}", &msg));
        println!("\x1B[2J\x1B[1;1H");
        match state {
            ServerState::Error { player, message } => {
                if player != self.nickname {
                    println!("Player: {}", player);
                }
                println!("Error: {}", message);
                Ok(())
            },
            ServerState::Info { message } => {
                println!("{}", message);
                Ok(())
            },
            ServerState::New { curr_player, board } => {
                println!("ᕕ(⌐■_■)ᕗ ♪♬ Game state update:");
                if curr_player == self.nickname {
                    println!("Here is the current board ↓");
                    println!("{}", board);
                    let position = self.ask_for_play();

                    let play = ClientState::Play {
                        nickname: &self.nickname,
                        position,
                        symbol: self.symbol.unwrap()
                    };

                    self.server.send(
                        Message::text(
                            serde_json::to_string(&play).unwrap()
                        )
                    ).unwrap();
                } else {
                    println!("It's {} turn, waiting a move... (❍ᴥ❍ʋ)", curr_player);
                }

                Ok(())
            },
            ServerState::Winner { player, board } => {
                if player == self.nickname {
                    println!("( ˘ ³˘)ノ°ﾟº❍｡ Congratulations! You're the winner!");
                } else {
                    println!("ε(´סּ︵סּ`)з Sorry, but {} won the match...", player);
                }
                println!("Here is the winner board: ↓");
                println!("{}", board);

                Ok(())
            },
            ServerState::Draw { message, board } => {
                println!("{message}");
                println!("Here is the board: ↓");
                println!("{}", board);
                Ok(())
            }
        }
    }
}

impl Client {
    pub fn new(server: Sender, symbol: Option<char>, mode: String, nickname: String, ) -> Self {
        Client { server, symbol, mode, nickname }
    }

    pub fn start(server_url: &str, symbol: Option<char>, mode: &str, nickname: &str) {
        connect(server_url, |out| Client::new(out, symbol, mode.to_string(), nickname.to_string())).unwrap_or_else(|err| {
            eprintln!("Failed to connect to the server: {:?}", err);
        });
    }

    pub fn send_message(&self, message: &str) -> Result<()> {
        self.server.send(message).unwrap();

        Ok(())
    }

    pub fn ask_for_play(&self) -> u8 {
        loop {
            print!("Your symbol is {}, select a position: ", self.symbol.unwrap());
            let _ = io::stdout().flush();
            let mut position = String::new();
            io::stdin().read_line(&mut position).unwrap();
            position = position.trim().to_string();

            if let Ok(position) = position.parse::<u8>() {
                break position;
            }

            println!("(ง •̀_•́)ง '{position}' is not a valid input, type only a number between or equal 1 and 9.");
        }
    }
}