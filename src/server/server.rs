use std::{cell::{Ref, RefCell}, rc::Rc, sync::Mutex};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ws::{listen, Error, Handler, Handshake, Message, Response, Result, Sender};

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
                self.clients.borrow_mut().push(Client::new(join_message.mode.clone(), join_message.nickname, self.out.clone()));
            } else if join_message.mode == "guest" {
                let guest_already_set = self.clients.borrow().iter().any(|client| client.mode == "guest");
                if guest_already_set == false {
                    let json = json!({
                        "content": format!("Hello {}, you has been connected successfully to the server.\nWe are initiating the game...", join_message.nickname)
                    }).to_string();
                    self.out.send(json).unwrap();
                    self.clients.borrow_mut().push(Client::new(join_message.mode, join_message.nickname, self.out.clone()));
                    self.send_to_host("The Player 2 has been connected, initiating the game...");
                    self.initialize_state();
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


        // TODO: Send to the client who play in the turn, and leave the handle to them...

        // println!("{:#?}", self.clients);
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

    fn initialize_state(&self) {
        *self.game_state.borrow_mut() = Some(GameState::new());
    }

    fn propagate_start(&self) {
        let start_message = format!("Welcome to my tic-tac-toe game! (˵ ͡° ͜ʖ ͡°˵)\nInitial board: ↓\n{}", self.game_state.borrow().as_ref().unwrap().board.visual_board.as_str());

        let json = json!({
            "content": start_message
        }).to_string();

        self.broadcast(Message::text(json));
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

type State = Rc<RefCell<Option<GameState>>>;
type Clients = Rc<RefCell<Vec<Client>>>;

pub fn start() {
    let game_state: State = State::new(RefCell::new(None));
    let clients: Clients = Clients::new(RefCell::new(vec![]));

    let local_address = local_ip().unwrap().to_string();
    println!("My addr: {}", local_ip().unwrap());
    listen(format!("{local_address}:8081"), |out| TicTacToeHandler {
            game_state: game_state.clone(),
            clients: clients.clone(),
            out
    }).unwrap();
}