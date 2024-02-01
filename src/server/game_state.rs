use std::{rc::Rc, sync::Mutex};

use crate::structs::board::Board;

use super::server::TicTacToeServer;

pub struct GameState {
    board: Board,
    server: Rc<Mutex<TicTacToeServer>>
}

impl GameState {
    pub fn new(server: Rc<Mutex<TicTacToeServer>>) -> GameState {
        let board = Board::new();
        
        GameState {
            board, server
        }
    }

    pub fn start(&self) -> Result<(), String> {
        let start_message = format!("Welcome to my tic-tac-toe game! (˵ ͡° ͜ʖ ͡°˵)\nInitial board: ↓\n{}", self.board.visual_board.as_str());
        self.server.lock().unwrap().broadcast(start_message.as_str()).unwrap();

        Ok(())
    }
}