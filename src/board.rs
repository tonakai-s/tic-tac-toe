use dyn_fmt::AsStrFormatExt;

#[derive(Debug)]
pub struct Board {
    pub board: Vec<char>,
    pub parsed_logic_board: Option<Vec<Vec<char>>>,
    pub visual_board: String,
    board_template: String,
}

impl Board {
    pub fn new() -> Board {
        let board_template = "
            | {} | {} | {} |
          -----------------
            | {} | {} | {} |
          -----------------
            | {} | {} | {} |
        ".to_string();

        let mut board = Board { board: ('1'..='9').collect(), parsed_logic_board: None, visual_board: String::new(), board_template };
        
        board.initialize_visual_board();

        board
    }

    fn initialize_visual_board(&mut self) {
        let board = self.board_template.format(&self.board);

        self.visual_board = board;
    }

    pub fn update_board(&mut self, user_play: u8, new_symbol: char) -> Result<(), String> {
        if self.play_already_throwed(user_play) {
            return Err(format!("Position {} is already in use!", user_play));
        }

        self.update_logic_board(user_play, new_symbol);
        self.update_visual_board();

        Ok(())
    }

    pub fn play_already_throwed(&mut self, user_play: u8) -> bool {
        let play_at_index = self.board.get((user_play - 1) as usize).unwrap().clone();
        
        play_at_index == '⬤' || play_at_index == '✖'
    }

    fn update_logic_board(&mut self, user_play: u8, new_symbol: char) {
        let position = self.board.get_mut((user_play - 1) as usize).unwrap();
        *position = new_symbol;

        self.update_parsed_logic_board()
    }

    fn update_visual_board(&mut self) {
        let board = self.board_template.format(&self.board);
        self.visual_board = board;
    }

    pub fn has_winner(&self) -> bool {
        if self.has_line_winner() == true {
            return true;
        }

        if self.has_column_winner() == true {
            return true;
        }

        if self.has_diagonal_winner() == true {
            return true;
        }

        false
    }

    fn has_line_winner(&self) -> bool {
        for line in self.parsed_logic_board.as_ref().unwrap() {
            if self.is_vector_winner(&line) == true {
                return true;
            }
        }

        false
    }

    fn has_column_winner(&self) -> bool {
        for i in 0..3 {
            let mut column_in_line: Vec<char> = vec![];
            for line in self.parsed_logic_board.as_ref().unwrap().iter() {
                column_in_line.push(line.get(i).unwrap().clone());
            }

            if self.is_vector_winner(&column_in_line) == true {
                return true;
            }
        }

        false
    }

    fn has_diagonal_winner(&self) -> bool {
        let mut diagonal_in_line: Vec<char> = vec![];
        let mut incremented_index = 0;

        for line in self.parsed_logic_board.as_ref().unwrap().iter() {
            diagonal_in_line.push(line.get(incremented_index).unwrap().clone());
            incremented_index += 1;
        }

        if self.is_vector_winner(&diagonal_in_line) == true {
            return true;
        }

        diagonal_in_line.clear();
        let decremented_index = 2;
        for (i, line) in self.parsed_logic_board.as_ref().unwrap().iter().enumerate() {
            diagonal_in_line.push(line.get(decremented_index - i).unwrap().clone());
        }

        if self.is_vector_winner(&diagonal_in_line) == true {
            return true;
        }

        false
    }

    fn is_vector_winner(&self, vector: &Vec<char>) -> bool {
        let first_element = vector.get(0).unwrap();

        for item in vector.iter() {
            if ( *item != *first_element ) || *item == ' ' {
                return false;
            }
        }

        true
    }

    fn update_parsed_logic_board(&mut self) {
        let mut parsed_logic_board = vec![];

        self.board.chunks(3).for_each(|chunk| {
            parsed_logic_board.push(chunk.to_vec())
        });

        self.parsed_logic_board = Some(parsed_logic_board);
    }
}