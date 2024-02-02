use std::{rc::Rc, sync::Mutex};

use crate::structs::board::Board;

#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    pub player_turn: String
}

impl GameState {
    pub fn new() -> GameState {
        let board = Board::new();
        
        GameState {
            board,
            player_turn: String::from("host")
        }
    }
}