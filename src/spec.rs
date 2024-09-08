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
pub enum SpecState<'a> {
    Join { mode: &'a str },
}

pub struct Spectator {
    server: Sender,
}

impl Handler for Spectator {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let join = SpecState::Join { mode: "spectator" };
        
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
                println!("Player: {}", player);
                println!("Error: {}", message);
                Ok(())
            },
            ServerState::Info { message } => {
                println!("{}", message);
                Ok(())
            },
            ServerState::New { curr_player, board } => {
                println!("ᕕ(⌐■_■)ᕗ ♪♬ Game state update:");
                println!("Player: {}", curr_player);
                println!("Here is the current board ↓");
                println!("{}", board);

                Ok(())
            },
            ServerState::Winner { player, board } => {
                println!("We have a winner: {player} ( ˘ ³˘)ノ°ﾟº❍｡");
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

impl Spectator {
    pub fn start(server_url: &str) {
        connect(server_url, |out| Spectator{ server: out }).unwrap_or_else(|err| {
            eprintln!("Failed to connect to the server: {:?}", err);
        });
    }

    pub fn send_message(&self, message: &str) -> Result<()> {
        self.server.send(message).unwrap();

        Ok(())
    }
}