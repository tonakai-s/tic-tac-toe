use std::{rc::Rc, sync::Mutex};
use local_ip_address::local_ip;
use ws::{listen, Error, Handler, Handshake, Message, Response, Result, Sender};

use super::game_state::GameState;

struct TicTacToeHandler {
    server: Rc<Mutex<TicTacToeServer>>,
    player_id: u8
}

impl Handler for TicTacToeHandler {
    fn on_request(&mut self, req: &ws::Request) -> Result<ws::Response> {
        if self.player_id > 2 {
            // TODO: The error need to be returned to the client
            Err(Error::new(
                ws::ErrorKind::Protocol,
                "Max players quantity already reached.",
            ))
        } else {
            Ok(Response::from_request(req).unwrap())
        }
    }

    // TODO: Where I will manage/store the global state???????
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        match self.player_id {
            1 => {self.server.lock().unwrap().broadcast("Waiting for player 2 join the game...").unwrap();},
            _ => {
                self.server.lock().unwrap().send_message_to_player(1, "Player 2 joined the game, initiating!");
                GameState::new(self.server.clone()).start().unwrap()
            }
        }

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // TODO: Send to the client who play in the turn, and leave the handle to them...
        println!("Server got a message: {msg}");
        println!("Received message from player {}: {}", self.player_id, msg.as_text().unwrap());

        let _ = self.server.lock().unwrap().broadcast(msg.as_text().unwrap());
        Ok(())
    }
}

pub struct TicTacToeServer {
    // TODO: Can be a HashSet of players if will be handled by the client
    player1: Option<Sender>,
    player2: Option<Sender>
}

impl TicTacToeServer {
    fn new() -> TicTacToeServer {
        TicTacToeServer {
            player1: None,
            player2: None
        }
    }

    fn add_player(&mut self, player: Sender) -> u8 {
        if self.player1.is_none() {
            self.player1 = Some(player);
            return 1;
        } else if self.player2.is_none() {
            self.player2 = Some(player);
            return 2;
        }

        return 99;
    }

    pub fn send_message_to_player(&self, player: u8, message: &str) {
        match player {
            1 => {self.player1.clone().unwrap().send(message).unwrap();},
            _ => {self.player2.clone().unwrap().send(message).unwrap();}
        }
        
    }

    pub fn broadcast(&self, message: &str) -> Result<()> {
        if self.player1.is_some() {
            let _ = self.player1.as_ref().unwrap().send(message);
        }

        if self.player2.is_some() {
            let _ = self.player2.as_ref().unwrap().send(message);
        }

        Ok(())
    }
}

pub fn start() {
    let tic_tac_toe_server = Rc::new(Mutex::new(TicTacToeServer::new()));

    let local_address = local_ip().unwrap().to_string();
    println!("My addr: {}", local_ip().unwrap());
    listen(format!("{local_address}:8081"), |out| TicTacToeHandler {
            server: tic_tac_toe_server.clone(),
            player_id: tic_tac_toe_server.lock().unwrap().add_player(out)
    }).unwrap();
}