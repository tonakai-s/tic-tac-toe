use crate::board::Board;

// TODO: Study more about lifetimes
#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    pub possible_players: Vec<String>,
    pub player_turn: String,
    pub turn: u8
}

impl GameState {
    pub fn new() -> GameState {
        let board = Board::new();
        
        GameState {
            board,
            possible_players: vec![],
            player_turn: String::new(),
            turn: 0
        }
    }

    pub fn define_initial_playable_state(&mut self) {
        self.player_turn = self.possible_players.get((self.turn % 2) as usize).unwrap().clone();
    }

    pub fn update_state(&mut self, position: u8, symbol: char) -> Result<(), String> {
        self.board.update_board(position, symbol)?;
        self.turn += 1;
        self.player_turn = self.possible_players.get( (self.turn % 2) as usize ).unwrap().to_string();

        Ok(())
    }
}