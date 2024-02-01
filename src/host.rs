use ws::{connect, Handler, Handshake, Message, Result, Sender};

pub struct Host {
    server: Sender,
    symbol: char,
    name: String
}

impl Handler for Host {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Connected to the server!");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}", msg.as_text().unwrap());
        Ok(())
    }
}

impl Host {
    fn new(server: Sender) -> Host {
        Host { symbol: '✖', name: String::from("player1"), server }
    }

    pub fn start(server_url: &str) {
        connect(server_url, |out| Host::new(out)).unwrap_or_else(|err| {
            eprintln!("Failed to connect to the server: {:?}", err);
        });
    }

    pub fn send_message(&self, message: &str) -> Result<()> {
        self.server.send(message).unwrap();

        Ok(())
    }
}