use std::{char::from_digit, ops::Range};
use dyn_fmt::AsStrFormatExt;

use crate::helpers::helpers;

#[derive(Debug)]
pub struct Board {
    pub board: Vec<char>,
    pub parsed_logic_board: Vec<Vec<char>>,
    pub visual_board: String,
    pub history: Vec<String>,
    board_template: String,
}

impl Board {
    pub fn new() -> Board {
        const POSSIBLE_PLAYS: Range<u32> = 1..10;
        let mut board: Vec<char> = vec![];
        for play in POSSIBLE_PLAYS{
            board.push(from_digit(play, 10).unwrap());
        }

        let board_template = "
            | {} | {} | {} |
          -----------------
            | {} | {} | {} |
          -----------------
            | {} | {} | {} |
        ".to_string();

        let mut board = Board { board, parsed_logic_board: vec![vec![]], visual_board: String::new(), history: vec![], board_template };
        
        board.parsed_logic_board = board.parse_logic_board();
        board.initialize_visual_board();

        board
    }

    fn initialize_visual_board(&mut self) {
        let board = self.board_template.format(&self.board);

        self.visual_board = board;
    }

    pub fn update_board(&mut self, user_play: u8, new_symbol: char) {
        self.update_logic_board(user_play, new_symbol);
        self.update_visual_board();
    }

    fn update_logic_board(&mut self, user_play: u8, new_symbol: char) {
        // let user_play_index = (user_play.to_digit(10).unwrap() - 1) as usize;
        let logic_board_reference = self.board.get_mut((user_play - 1) as usize).unwrap();
        *logic_board_reference = new_symbol;
    }

    fn update_visual_board(&mut self) {
        self.add_visual_board_history();

        let board = self.board_template.format(&self.board);
        self.visual_board = board;
    }

    fn add_visual_board_history(&mut self) {
        self.history.push(self.visual_board.clone());
    }

    pub fn play_already_throwed(&mut self, user_play: char) -> bool {
        let user_play_index = (user_play.to_digit(10).unwrap() - 1) as usize;
        let play_at_index = self.board.get(user_play_index).unwrap().clone();
        
        play_at_index == '⬤' || play_at_index == '✖'
    }

    pub fn has_winner(&self) -> bool {
        let parsed_logic_board = self.parse_logic_board();
        if self.has_line_winner(&parsed_logic_board) == true {
            return true;
        }

        if self.has_column_winner(&parsed_logic_board) == true {
            return true;
        }

        if self.has_diagonal_winner(&parsed_logic_board) == true {
            return true;
        }

        false
    }

    fn has_line_winner(&self, parsed_logic_board: &Vec<Vec<char>>) -> bool {
        for line in parsed_logic_board {
            if helpers::is_vector_winner(line) == true {
                return true;
            }
        }

        false
    }

    fn has_column_winner(&self, parsed_logic_board: &Vec<Vec<char>>) -> bool {
        for i in 0..3 {
            let mut column_in_line: Vec<char> = vec![];
            for line in parsed_logic_board.iter() {
                column_in_line.push(line.get(i).unwrap().clone());
            }

            if helpers::is_vector_winner(&column_in_line) == true {
                return true;
            }
        }

        false
    }

    fn has_diagonal_winner(&self, parsed_logic_board: &Vec<Vec<char>>) -> bool {
        let mut diagonal_in_line: Vec<char> = vec![];
        let mut incremented_index = 0;

        for line in parsed_logic_board.iter() {
            diagonal_in_line.push(line.get(incremented_index).unwrap().clone());
            incremented_index += 1;
        }

        if helpers::is_vector_winner(&diagonal_in_line) == true {
            return true;
        }

        diagonal_in_line.clear();
        let decremented_index = 2;
        for (i, line) in parsed_logic_board.iter().enumerate() {
            diagonal_in_line.push(line.get(decremented_index - i).unwrap().clone());
        }

        if helpers::is_vector_winner(&diagonal_in_line) == true {
            return true;
        }

        false
    }

    fn parse_logic_board(&self) -> Vec<Vec<char>> {
        let mut parsed_logic_board = vec![];

        self.board.chunks(3).for_each(|chunk| {
            parsed_logic_board.push(chunk.to_vec())
        });

        parsed_logic_board
    }
}