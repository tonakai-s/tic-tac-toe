use std::vec;
use ws::Sender;
use crate::board::Board;

#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    pub players: Vec<Player>,
    pub specs: Vec<Spectator>,
    pub player_turn: String,
    pub turn: u8
}

#[derive(PartialEq, Debug)]
pub struct Player {
    pub mode: String,
    pub nickname: String,
    pub out: Sender
}

#[derive(Debug)]
pub struct Spectator {
    pub out: Sender
}

impl GameState {
    pub fn new() -> Self {
        let board = Board::new();
        
        Self {
            board,
            players: vec![],
            specs: vec![],
            player_turn: String::new(),
            turn: 0
        }
    }

    pub fn define_initial_playable_state(&mut self) {
        self.player_turn = self.players.get((self.turn % 2) as usize).unwrap().nickname.clone();
    }

    pub fn update_state(&mut self, position: u8, symbol: char) -> Result<(), String> {
        self.board.update_board(position, symbol)?;
        self.turn += 1;
        self.player_turn = self.players.get( (self.turn % 2) as usize ).unwrap().nickname.clone();

        Ok(())
    }

    pub fn curr_player(&self) -> &'_ str {
        &self.player_turn
    }

    pub fn add_player(&mut self, mode: String, nickname: String, out: Sender) {
        self.players.push(
            Player {
                mode, nickname, out
            }
        );
    }
    
    pub fn add_spec(&mut self, out: Sender) {
        self.specs.push(
            Spectator {
                out
            }
        )
    }
}