use std::{io::Write, borrow::BorrowMut};

struct Board {
    visual_board: String,
    logic_board: Vec<Vec<char>>,
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

        let logic_board: Vec<Vec<char>> = vec![vec![' ';3];3];

        Board { visual_board, logic_board, history: vec![] }
    }

    fn update_visual_board(&mut self, user_play: String, new_symbol: char) {
        self.history.push(self.visual_board.clone());
        self.visual_board = self.visual_board.replace(&user_play, &new_symbol.to_string());
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

    print!("\x1B[2J");
    println!("Board initiated! ↓");
    println!("{}", board.visual_board);

    loop {
        let current_player: &mut Player = players.get_mut(play_counter % 2).unwrap();
        println!("{} turn, with symbol: {}[2J", current_player.name, current_player.symbol);
        if current_player.previous_play != ' ' {
            println!("{} previously throwed {}", current_player.name, current_player.previous_play);
        }

        print!("Inform the number of your play: ");
        std::io::stdout().flush().unwrap();

        let user_play = get_play_input();

        let mut user_play_int: u8 = 0;

        match is_valid_input(&user_play) {
            Ok(user_play) => {
                user_play_int = user_play;
            },
            Err(error) => {
                println!("{}", error);
                continue;
            }
        }

        current_player.previous_play = user_play.chars().next().unwrap();
        board.update_visual_board(user_play, current_player.symbol);

        println!("Board updated! ↓");
        println!("{}", board.visual_board);

        if has_winner == true || play_counter == 9 {
            break;
        }

        play_counter += 1;
    }
}

fn is_valid_input(input: &str) -> Result<u8, String> {
    match input.parse::<u8>() {
        Ok(user_play) => {
            if user_play > 0 && user_play < 10 {
                return Ok(user_play);
            }

            return Err(String::from("Play need to be between 1 and 9."));
        }

        Err(_) => Err(String::from("Play need to contain only number."))
    }
}

fn get_play_input() -> String {
    let mut user_play = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut user_play).unwrap();
    user_play.pop();

    user_play
}