use std::io::Write;

use tic_tac_toe::structs::{board::Board, player::Player};

fn main() {
    let mut board = Board::new();

    let mut players: Vec<Player> = vec![
        Player::new('⬤', String::from("Player 1")),
        Player::new('✖', String::from("Player 2"))
    ];

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

        board.update_board(user_play, current_player.symbol);

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
    print!("Inform the number of your play: ");
    std::io::stdout().flush().unwrap();

    let mut user_play = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut user_play).unwrap();

    user_play.chars().next().unwrap()
}

fn clear_terminal() {
    print!("\x1B[2J");
}