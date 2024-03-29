use std::{cell::RefCell, rc::Rc};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ws::{listen, Handler, Message, Result, Sender};

use super::game_state::GameState;

pub fn start() {
    let game_state: State = State::new(RefCell::new(GameState::new()));
    // TODO: Players and Specs go to the same Vector?
    let clients: Clients = Clients::new(RefCell::new(vec![]));
    let local_address = local_ip().unwrap();
    listen(format!("{local_address}:8081"), |out| TicTacToeHandler {
            game_state: game_state.clone(),
            clients: clients.clone(),
            out
    }).unwrap();
}

#[derive(Debug)]
struct TicTacToeHandler {
    game_state: State,
    clients: Clients,
    out: Sender
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

impl Handler for TicTacToeHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Ok(join_message) = serde_json::from_str::<JoinMessage>(&msg.to_string()) {
            match join_message.mode.as_str() {
                "host" => {
                    let json = json!({
                        "content": format!("(ᕗ ͠° ਊ ͠° )ᕗ Hello {}, the server has been successfully created.\nʕ·͡ᴥ·ʔ We are waiting the Player2 (Guest) connect...\n⊂(◉‿◉)つ To access as a guest, call with these arguments '--mode=guest --nick=NICKNAME --addr={}'", join_message.nickname, local_ip().unwrap())
                    }).to_string();
                    self.out.send(Message::text(json)).unwrap();
                    self.clients.borrow_mut().push(Client::new(join_message.mode.clone(), join_message.nickname.clone(), self.out.clone()));
                    self.game_state.borrow_mut().possible_players.push(join_message.nickname);
                },
                "guest" => {
                    let guest_already_set = self.clients.borrow().iter().any(|client| client.mode == "guest");
                    if !guest_already_set {
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
                        self.out.close_with_reason(ws::CloseCode::Normal, "The game already have a Guest.").unwrap();
                    }
                },
                _ => self.clients.borrow_mut().push(Client::new(join_message.mode, join_message.nickname, self.out.clone()))
            }

            return Ok(());
        }

        if let Ok(play) = serde_json::from_str::<Play>(&msg.to_string()) {
            if let Err(error) = self.game_state.borrow_mut().update_state(play.position, play.symbol) {
                let error_state = json!({
                    "nickname": play.nickname,
                    "player_error": format!("(ง •̀_•́)ง {}", error),
                    "others_error": format!("{} inserted a in use position! Still waiting... (❍ᴥ❍ʋ)", play.nickname)
                }).to_string();

                self.broadcast(Message::text(error_state));

                return Ok(());
            };

            if self.game_state.borrow().board.has_winner() {
                let winner_state = json!({
                    "winner": play.nickname,
                    "visual_board": self.game_state.borrow().board.visual_board
                }).to_string();

                self.broadcast(Message::text(winner_state));

                return Ok(());
            }
            
            let new_state = json!({
                "turn_nickname": self.game_state.borrow().player_turn,
                "visual_board": self.game_state.borrow().board.visual_board
            }).to_string();
    
            self.broadcast(Message::text(new_state));
            return Ok(())
        }

        Ok(())
    }

    // TODO: Remove client when closed connection
    // TODO: Close all connections when Host close the connection
    // fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        
    // }
}

// TODO: Be able to send a command to the client??
impl TicTacToeHandler {
    fn send_to_host(&self, msg: &str) {
        let json = json!({
            "content": msg
        }).to_string();
        self.clients.borrow().first().unwrap().out.send(Message::text(json)).unwrap();
    }

    fn propagate_start(&self) {
        let start_message = String::from("Welcome to my tic-tac-toe game! (˵ ͡° ͜ʖ ͡°˵)");

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