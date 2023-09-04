use crate::helpers::helpers;

pub struct Board {
    pub visual_board: String,
    pub logic_board: Vec<char>,
    pub history: Vec<String>
}

impl Board {
    pub fn new() -> Board {
        let visual_board = r#"
            | 1 | 2 | 3 |
           --------------
            | 4 | 5 | 6 |
           ---------------
            | 7 | 8 | 9 |
        "#.to_string();

        let logic_board: Vec<char> = vec![' ';9];

        Board { visual_board, logic_board, history: vec![] }
    }

    pub fn update_visual_board(&mut self, user_play: char, new_symbol: char) {
        self.history.push(self.visual_board.clone());
        self.visual_board = self.visual_board.replace(&user_play.to_string(), &new_symbol.to_string());
    }

    pub fn update_logic_board(&mut self, user_play: char, new_symbol: char) {
        let user_play_index = (user_play.to_digit(10).unwrap() - 1) as usize;
        let logic_board_reference = self.logic_board.get_mut(user_play_index).unwrap();
        *logic_board_reference = new_symbol;
    }

    pub fn play_already_throwed(&mut self, user_play: char) -> bool {
        let user_play_index = (user_play.to_digit(10).unwrap() - 1) as usize;
        if self.logic_board.get(user_play_index).unwrap().clone() != ' '{
            return true;
        }

        false
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

        self.logic_board.chunks(3).for_each(|chunk| {
            parsed_logic_board.push(chunk.to_vec())
        });

        parsed_logic_board
    }
}