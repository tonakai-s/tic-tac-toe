use std::{cell::RefCell, rc::Rc};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use ws::{listen, Handler, Message, Result, Sender};
use super::game_state::GameState;

#[derive(Serialize, Deserialize)]
pub enum ServerState<'a> {
    Error { player: &'a str, message: String },
    Info { message: &'a str },
    New { curr_player: &'a str, board: &'a str},
    Winner { player: String, board: &'a str },
    Draw { message: String, board: &'a str }
}

#[derive(Serialize, Deserialize)]
pub enum PlayerState {
    Join { mode: String, nickname: String },
    Play { nickname: String, position: u8, symbol: char }
}

#[derive(Serialize, Deserialize)]
pub enum SpecState {
    Join { mode: String },
}

pub fn start() {
    let game_state: State = State::new(RefCell::new(GameState::new()));
    let local_address = local_ip().unwrap();
    listen(format!("{local_address}:8081"), |out| TicTacToeHandler {
            game_state: game_state.clone(),
            out
    }).unwrap();
}

type State = Rc<RefCell<GameState>>;
#[derive(Debug)]
struct TicTacToeHandler {
    game_state: State,
    out: Sender
}

impl Handler for TicTacToeHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let message = serde_json::from_str::<PlayerState>(&msg.to_string()).unwrap();
        match message {
            PlayerState::Join { mode, nickname } => {
                match mode.as_str() {
                    "host" => {
                        let greetings = ServerState::Info {
                            message: &format!("(ᕗ ͠° ਊ ͠° )ᕗ Hello {}, the server has been successfully created.\nʕ·͡ᴥ·ʔ We are waiting the Player2 (Guest) connect...\n⊂(◉‿◉)つ To access as a guest, call with these arguments '--mode=guest --nick=NICKNAME --addr={}'\n⊂(◉‿◉)つ To access as a spectator, call with these arguments '--mode=spec --addr={}'", nickname, local_ip().unwrap(), local_ip().unwrap())
                        };
                        self.out.send(
                            Message::text(
                                serde_json::to_string(&greetings).unwrap()
                            )
                        ).unwrap();
                        self.game_state.borrow_mut().add_player(mode, nickname, self.out.clone());
                    },
                    "guest" => {
                        let guest_already_set = self.game_state.borrow().players.iter().any(|player| player.mode == "guest");
                        if !guest_already_set {
                            let greetings = ServerState::Info {
                                message: &format!("Hello {}, you has been connected successfully to the server.\nWe are initiating the game...", nickname)
                            };
                            self.out.send(
                                Message::text(
                                    serde_json::to_string(&greetings).unwrap()
                                )
                            ).unwrap();

                            self.game_state.borrow_mut().add_player(mode, nickname, self.out.clone());
                            self.send_to_host("The Player 2 has been connected, initiating the game...");
                            self.game_state.borrow_mut().define_initial_playable_state();
                            self.propagate_start();
                        } else {
                            self.out.close_with_reason(ws::CloseCode::Normal, "The game already have a Guest.").unwrap();
                        }
                    },
                    _ => {}
                }
            },
            PlayerState::Play { nickname, position, symbol } => {
                if nickname != self.game_state.borrow().curr_player() {
                    return Ok(());
                }

                let state_update = self.game_state.borrow_mut().update_state(position, symbol);
                if state_update.is_err() {
                    let error_state = ServerState::Error {
                        player: &self.game_state.borrow().player_turn,
                        message: format!("(ง •̀_•́)ง {}", "???????")
                    };

                    assert!(serde_json::to_string(&error_state).is_ok());

                    self.broadcast(
                        Message::text(
                            serde_json::to_string(&error_state).unwrap()
                        )
                    );
                };

                if self.game_state.borrow().board.has_winner() {
                    let winner = ServerState::Winner {
                        player: nickname,
                        board: &self.game_state.borrow().board.visual_board
                    };
                    self.broadcast(
                        Message::text(
                            serde_json::to_string(&winner).unwrap()
                        )
                    );

                    return Ok(());
                }

                if self.game_state.borrow().board.is_draw() {
                    let draw = ServerState::Draw {
                        message: "Sorry, but the game is a draw.".to_string(),
                        board: &self.game_state.borrow().board.visual_board
                    };
                    self.broadcast(
                        Message::text(
                            serde_json::to_string(&draw).unwrap()
                        )
                    );
                }

                let new_state = ServerState::New {
                    curr_player: &self.game_state.borrow().player_turn,
                    board: &self.game_state.borrow().board.visual_board
                };
        
                self.broadcast(
                    Message::text(
                        serde_json::to_string(&new_state).unwrap()
                    )
                );
            }
        }
        let message = serde_json::from_str::<SpecState>(&msg.to_string()).unwrap();
        match message {
            SpecState::Join { mode: _ } => {
                self.game_state.borrow_mut().add_spec(self.out.clone())
            }
        }

        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        dbg!(format!("{:?} - {}", code, reason));
    }

    // TODO: Remove client when closed connection
    // TODO: Close all connections when Host close the connection
    // fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        
    // }
}

// TODO: Be able to send a command to the client??
impl TicTacToeHandler {
    fn send_to_host(&self, msg: &str) {
        let message = ServerState::Info { message: msg };
        self.game_state.borrow().players.first().unwrap().out.send(
            Message::text(
                serde_json::to_string(&message).unwrap()
            )
        ).unwrap();
    }

    fn propagate_start(&self) {
        let welcome = ServerState::Info {
            message: "Welcome to my tic-tac-toe game! (˵ ͡° ͜ʖ ͡°˵)"
        };
        self.broadcast(
            Message::text(
                serde_json::to_string(&welcome).unwrap()
            )
        );

        let start = ServerState::New {
            curr_player: &self.game_state.borrow().player_turn,
            board: &self.game_state.borrow().board.visual_board
        };
        self.broadcast(
            Message::text(
                serde_json::to_string(&start).unwrap()
            )
        );
    }

    fn broadcast(&self, msg: Message) {
        for player in self.game_state.borrow().players.iter() {
            player.out.send(msg.clone()).unwrap();
        }

        for spec in self.game_state.borrow().specs.iter() {
            spec.out.send(msg.clone()).unwrap();
        }
    }
}