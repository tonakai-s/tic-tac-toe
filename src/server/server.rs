use std::{cell::{Ref, RefCell}, rc::Rc, sync::Mutex};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ws::{listen, Error, Handler, Handshake, Message, Response, Result, Sender};

use crate::structs::board::Board;

use super::{game::Game, game_state::GameState};

#[derive(Debug)]
struct TicTacToeHandler {
    game_state: State,
    clients: Clients,
    out: Sender
}

impl Handler for TicTacToeHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Ok(join_message) = serde_json::from_str::<JoinMessage>(&msg.to_string()) {
            // TODO: Refactor
            if join_message.mode == "host" {
                let json = json!({
                    "content": format!("Hello {}, the server has been successfully created.\nWe are waiting the Player2 (Guest) connect...", join_message.nickname)
                }).to_string();
                self.out.send(Message::text(json)).unwrap();
                self.clients.borrow_mut().push(Client::new(join_message.mode.clone(), join_message.nickname.clone(), self.out.clone()));
                self.game_state.borrow_mut().possible_players.push(join_message.nickname);
            } else if join_message.mode == "guest" {
                let guest_already_set = self.clients.borrow().iter().any(|client| client.mode == "guest");
                if guest_already_set == false {
                    let json = json!({
                        "content": format!("Hello {}, you has been connected successfully to the server.\nWe are initiating the game...", join_message.nickname)
                    }).to_string();
                    self.out.send(json).unwrap();
                    self.clients.borrow_mut().push(Client::new(join_message.mode, join_message.nickname.clone(), self.out.clone()));
                    self.game_state.borrow_mut().possible_players.push(join_message.nickname);
                    self.send_to_host("The Player 2 has been connected, initiating the game...");
                    self.game_state.borrow_mut().define_initial_playable_state();
                    self.propagate_start();
                } else {
                    // TODO: Send a message
                    self.out.close(ws::CloseCode::Normal).unwrap();
                }
            } else {
                self.clients.borrow_mut().push(Client::new(join_message.mode, join_message.nickname, self.out.clone()));
            }
            return Ok(());
        }

        // TODO: Where validate if the position is valid???????
        if let Ok(play) = serde_json::from_str::<Play>(&msg.to_string()) {
            self.game_state.borrow_mut().update_state(play.position, play.symbol);
            
            let new_state = json!({
                "turn_nickname": self.game_state.borrow().player_turn,
                "visual_board": self.game_state.borrow().board.visual_board
            }).to_string();
    
            self.broadcast(Message::text(new_state));
            return Ok(())
        }

        // TODO: Send to the client who play in the turn, and leave the handle to them...
        Ok(())
    }

    // TODO: Remove client when closed connection
    // fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        
    // }
}

// TODO: Be able to send a command to the client
impl TicTacToeHandler {
    fn send_to_host(&self, msg: &str) {
        let json = json!({
            "content": msg
        }).to_string();
        self.clients.borrow().get(0).unwrap().out.send(Message::text(json)).unwrap();
    }

    fn propagate_start(&self) {
        let start_message = format!("Welcome to my tic-tac-toe game! (˵ ͡° ͜ʖ ͡°˵)");

        let json = json!({
            "content": start_message
        }).to_string();

        self.broadcast(Message::text(json));

        let start_state = json!({
            "turn_nickname": self.game_state.borrow().player_turn,
            "visual_board": self.game_state.borrow().board.visual_board
        }).to_string();

        self.broadcast(Message::text(start_state));
    }

    fn broadcast(&self, msg: Message) {
        for client in self.clients.borrow().iter() {
            // TODO: This clone has too much performance cost?
            client.out.send(msg.clone()).unwrap();
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Play {
    nickname: String,
    position: u8,
    symbol: char
}

#[derive(Serialize, Deserialize, Debug)]
struct JoinMessage {
    mode: String,
    nickname: String
}

#[derive(PartialEq, Debug)]
pub struct Client {
    mode: String,
    nickname: String,
    out: Sender
}

impl Client {
    fn new(mode: String, nickname: String, out: Sender) -> Self {
        Client { mode, nickname, out }
    }
}

type State = Rc<RefCell<GameState>>;
type Clients = Rc<RefCell<Vec<Client>>>;

pub fn start() {
    let game_state: State = State::new(RefCell::new(GameState::new()));
    let clients: Clients = Clients::new(RefCell::new(vec![]));

    let local_address = local_ip().unwrap().to_string();
    println!("My addr: {}", local_ip().unwrap());
    listen(format!("{local_address}:8081"), |out| TicTacToeHandler {
            game_state: game_state.clone(),
            clients: clients.clone(),
            out
    }).unwrap();
}