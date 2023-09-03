use std::io::Write;
use tic_tac_toe::helpers::helpers;

struct Board {
    visual_board: String,
    logic_board: Vec<char>,
    history: Vec<String>
}

impl Board {
    fn new() -> Board {
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

    fn update_visual_board(&mut self, user_play: char, new_symbol: char) {
        self.history.push(self.visual_board.clone());
        self.visual_board = self.visual_board.replace(&user_play.to_string(), &new_symbol.to_string());
    }

    fn update_logic_board(&mut self, user_play: char, new_symbol: char) {
        let user_play_index = (user_play.to_digit(10).unwrap() - 1) as usize;
        let logic_board_reference = self.logic_board.get_mut(user_play_index).unwrap();
        *logic_board_reference = new_symbol;
    }

    fn play_already_throwed(&mut self, user_play: char) -> bool {
        let user_play_index = (user_play.to_digit(10).unwrap() - 1) as usize;
        if self.logic_board.get(user_play_index).unwrap().clone() != ' '{
            return true;
        }

        false
    }

    fn has_winner(&self) -> bool {
        let parsed_logic_board = self.parse_logic_board();
        if self.has_line_winner(&parsed_logic_board) == true {
            return true;
        }

        false
    }

    fn has_line_winner(&self, logic_board: &Vec<Vec<char>>) -> bool {
        for line in logic_board {
            if helpers::is_line_winner(line) == true {
                return true;
            }
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

struct Player {
    symbol: char,
    name: String,
    previous_play: char,
    history: Vec<char>
}

impl Player {
    fn new(symbol: char, name: String) -> Player{
        Player { symbol, name , previous_play: ' ', history: vec![] }
    }
}

fn main() {
    let mut board = Board::new();

    let mut players: Vec<Player> = vec![
        Player::new('⬤', String::from("Player 1")),
        Player::new('✖', String::from("Player 2"))
    ];

    let has_winner = false;
    let mut play_counter = 0;

    clear_terminal();
    println!("Board initiated! ↓");
    println!("{}", board.visual_board);

    loop {
        let current_player: &mut Player = players.get_mut(play_counter % 2).unwrap();
        println!("{} turn, with symbol: {}", current_player.name, current_player.symbol);
        if current_player.previous_play != ' ' {
            println!("{} previously throwed {}", current_player.name, current_player.previous_play);
        }

        print!("Inform the number of your play: ");
        std::io::stdout().flush().unwrap();

        let user_play = get_play_input();

        match is_valid_input(user_play) {
            Ok(_) => (),
            Err(error) => {
                println!("{}", error);
                continue;
            }
        }

        let play_already_throwed = board.play_already_throwed(user_play);
        if play_already_throwed == true {
            println!("This place has been used, choose another!");
            continue;
        }

        current_player.previous_play = user_play;

        board.update_visual_board(user_play, current_player.symbol);
        board.update_logic_board(user_play, current_player.symbol);

        println!("Logic board updated! ↓");
        println!("{:?}", board.logic_board);

        println!("Board updated! ↓");
        println!("{}", board.visual_board);

        if board.has_winner() == true {
            println!("Congratulations! {} is the winner!", current_player.name);
            break;
        }

        if play_counter == 9 {
            println!("The match ended with a draw!");
            break;
        }

        board.parse_logic_board();
        play_counter += 1;
    }
}

fn is_valid_input(input: char) -> Result<(), String> {
    match input.to_digit(10) {
        Some(user_play) => {
            if user_play > 0 && user_play < 10 {
                return Ok(());
            }

            return Err(String::from("Play need to be between 1 and 9."));
        }

        None => Err(String::from("Play need to contain only number."))
    }
}

fn get_play_input() -> char {
    let mut user_play = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut user_play).unwrap();

    user_play.chars().next().unwrap()
}

fn clear_terminal() {
    print!("\x1B[2J");
}