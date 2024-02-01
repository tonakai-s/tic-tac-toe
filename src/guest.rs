use ws::{connect, Handler, Handshake, Message, Result, Sender};

pub struct Guest {
    server: Sender,
    symbol: char,
    name: String
}

impl Handler for Guest {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Connected to the server!");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}", msg.as_text().unwrap());
        Ok(())
    }
}

impl Guest {
    pub fn new(server: Sender) -> Guest {
        Guest { symbol: '⬤', name: String::from("player2"), server }
    }

    pub fn start(server_url: &str) {
        connect(server_url, |out| Guest::new(out)).unwrap_or_else(|err| {
            eprintln!("Failed to connect to the server: {:?}", err);
        });
    }

    pub fn send_message(&self, message: &str) -> Result<()> {
        self.server.send(message).unwrap();

        Ok(())
    }
}