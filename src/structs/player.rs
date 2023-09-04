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
}