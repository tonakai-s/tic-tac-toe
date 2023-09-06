use crate::structs::{player::Player, board::Board};

pub struct Game {
    pub players: Vec<Player>,
    pub board: Board,
    pub play_counter: usize
}

impl Game {
    pub fn new() -> Game {
        let board = Board::new();

        let players: Vec<Player> = vec![
            Player::new('⬤', String::from("Player 1")),
            Player::new('✖', String::from("Player 2"))
        ];

        Game { players, board, play_counter: 0 }
    }

    pub fn run(&mut self) {
        Game::clear_terminal();

        println!("Welcome to my tic-tac-toe game! (˵ ͡° ͜ʖ ͡°˵)");

        println!("Board: ↓");
        println!("{}", self.board.visual_board);

        loop {
            let current_player: &mut Player = self.players.get_mut(self.play_counter % 2).unwrap();
            println!("\n{} turn, with symbol: {}", current_player.name, current_player.symbol);

            if current_player.previous_play != ' ' {
                println!("{} previously throwed {}", current_player.name, current_player.previous_play);
            }

            let play = match Player::get_play() {
                Ok(play) => play,
                Err(error) => {
                    println!("{}", error);
                    continue;
                }
            };

            if self.board.play_already_throwed(play) == true {
                println!("This place has been used, choose another!");
                continue;
            }

            current_player.previous_play = play;

            self.board.update_board(play, current_player.symbol);

            println!("Board: ↓");
            println!("{}", self.board.visual_board);

            if self.board.has_winner() == true {
                println!("Congratulations! {} is the winner! (ﾉ☉ヮ⚆)ﾉ ⌒*:･ﾟ✧", current_player.name);
                break;
            }

            if self.play_counter == 9 {
                println!("The match ended with a draw!");
                break;
            }

            self.play_counter += 1;
        }
    }

    fn clear_terminal() {
        print!("\x1B[2J");
    }
}