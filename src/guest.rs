use serde::{Deserialize, Serialize};
use serde_json::json;
use ws::{connect, Handler, Handshake, Message, Result, Sender};

pub struct Guest {
    server: Sender,
    symbol: char,
    mode: String,
    nickname: String
}

#[derive(Deserialize, Serialize)]
pub struct CommonInfo {
    content: String
}

impl Handler for Guest {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let join_data = json!({
            "mode": "guest",
            "nickname": "renas2",
        }).to_string();

        self.server.send(Message::text(join_data)).unwrap();
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Ok(message) = serde_json::from_str::<CommonInfo>(&msg.to_string()) {
            println!("{}", message.content);
        }
        
        Ok(())
    }
}

impl Guest {
    pub fn new(server: Sender) -> Guest {
        Guest { symbol: '⬤', mode: String::from("guest"), server, nickname: String::from("renas2") }
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