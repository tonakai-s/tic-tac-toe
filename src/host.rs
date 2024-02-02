use serde::{Deserialize, Serialize};
use serde_json::json;
use ws::{connect, Handler, Handshake, Message, Result, Sender};

pub struct Host {
    server: Sender,
    symbol: char,
    mode: String,
    nickname: String
}

#[derive(Deserialize, Serialize)]
pub struct CommonInfo {
    content: String
}

impl Handler for Host {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let join_data = json!({
            "mode": self.mode,
            "nickname": self.nickname
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

impl Host {
    fn new(server: Sender, nickname: String) -> Host {
        Host { symbol: '✖', mode: String::from("host"), server, nickname }
    }

    pub fn start(server_url: &str, nickname: &str) {
        connect(server_url, |out| Host::new(out, nickname.to_string())).unwrap_or_else(|err| {
            eprintln!("Failed to connect to the server: {:?}", err);
        });
    }

    pub fn send_message(&self, message: &str) -> Result<()> {
        self.server.send(message).unwrap();

        Ok(())
    }
}