pub struct Guest {
    symbol: char,
    name: String
}

impl Guest {
    pub fn new() -> Guest {
        Guest { symbol: '⬤', name: String::from("guest") }
    }
}