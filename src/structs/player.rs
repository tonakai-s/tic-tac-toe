use std::io::Write;

pub struct Player {
    pub symbol: char,
    pub name: String,
    pub previous_play: char,
    pub history: Vec<char>
}

impl Player {
    pub fn new(symbol: char, name: String) -> Player{
        Player { symbol, name , previous_play: ' ', history: vec![] }
    }

    pub fn get_play() -> Result<char, String> {
        let input = Player::get_play_input();
        Player::is_valid_input(input)
    }

    fn get_play_input() -> char {
        print!("Inform the number of your play: ");
        std::io::stdout().flush().unwrap();

        let mut user_play = String::new();
        let stdin = std::io::stdin();

        stdin.read_line(&mut user_play).unwrap();

        user_play.chars().next().unwrap()
    }

    fn is_valid_input(input: char) -> Result<char, String> {
        match input.to_digit(10) {
            Some(user_play) => {
                if user_play > 0 && user_play < 10 {
                    return Ok(input);
                }
    
                return Err(String::from("Play need to be between 1 and 9."));
            }
    
            None => Err(String::from("Play need to contain only number."))
        }
    }
}