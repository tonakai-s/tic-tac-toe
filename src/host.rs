use std::io::{self, Write};

use serde::{Deserialize, Serialize};
use serde_json::json;
use ws::{connect, Handler, Handshake, Message, Result, Sender};

use crate::client::Client;

pub struct Host {
    client: Client,
}



// #[derive(Deserialize, Serialize)]
// pub struct CommonInfo {
//     content: String
// }

// #[derive(Deserialize, Serialize)]
// struct NewGameState {
//     turn_nickname: String,
//     visual_board: String,
// }

// #[derive(Deserialize, Serialize)]
// struct ErrorState {
//     nickname: String,
//     player_error: String,
//     others_error: String
// }

// #[derive(Deserialize, Serialize)]
// struct Winner {
//     winner: String,
//     visual_board: String
// }

// impl Handler for Host {
//     fn on_open(&mut self, _: Handshake) -> Result<()> {
//         let join_data = json!({
//             "mode": self.mode,
//             "nickname": self.nickname
//         }).to_string();

//         self.server.send(Message::text(join_data)).unwrap();
//         Ok(())
//     }

//     fn on_message(&mut self, msg: Message) -> Result<()> {
//         if let Ok(message) = serde_json::from_str::<CommonInfo>(&msg.to_string()) {
//             println!("{}", message.content);
//         }

//         if let Ok(new_state) = serde_json::from_str::<NewGameState>(&msg.to_string()) {
//             println!("* A NEW GAME STATE HAS BEEN RECEIVED!");
//             if new_state.turn_nickname == self.nickname {
//                 println!("Here is the current board ↓");
//                 println!("{}", new_state.visual_board);
//                 let position = self.ask_for_play();

//                 let play_json = json!({
//                     "nickname": self.nickname,
//                     "position": position,
//                     "symbol": self.symbol
//                 }).to_string();

//                 self.server.send(play_json).unwrap();
//             } else {
//                 println!("It's {} turn, waiting a play... (❍ᴥ❍ʋ)", new_state.turn_nickname);
//             }
//         }

//         if let Ok(error) = serde_json::from_str::<ErrorState>(&msg.to_string()) {
//             if error.nickname == self.nickname {
//                 println!("{}", error.player_error);
//                 let position = self.ask_for_play();
//                 let play_json = json!({
//                     "nickname": self.nickname,
//                     "position": position,
//                     "symbol": self.symbol
//                 }).to_string();

//                 self.server.send(play_json).unwrap();
//             } else {
//                 println!("{}", error.others_error);
//             }
//         }

//         if let Ok(winner) = serde_json::from_str::<Winner>(&msg.to_string()) {
//             if winner.winner == self.nickname {
//                 println!("( ˘ ³˘)ノ°ﾟº❍｡ Congratulations! You're the winner!");
//             } else {
//                 println!("ε(´סּ︵סּ`)з Sorry, but {} won the match...", winner.winner);
//             }
//         }

//         Ok(())
//     }
// }

// impl Host {
//     fn new(server: Sender, nickname: String) -> Host {
//         Host { symbol: '✖', mode: String::from("host"), server, nickname }
//     }

//     pub fn start(server_url: &str, nickname: &str) {
//         connect(server_url, |out| Host::new(out, nickname.to_string())).unwrap_or_else(|err| {
//             eprintln!("Failed to connect to the server: {:?}", err);
//         });
//     }

//     pub fn send_message(&self, message: &str) -> Result<()> {
//         self.server.send(message).unwrap();

//         Ok(())
//     }

//     pub fn ask_for_play(&self) -> u8 {
//         loop {
//             print!("Your symbol is {}, select a position: ", self.symbol);
//             let _ = io::stdout().flush();
//             let mut position = String::new();
//             io::stdin().read_line(&mut position).unwrap();
//             position = position.trim().to_string();

//             if let Ok(position) = position.parse::<u8>() {
//                 if Vec::from_iter(1..10).contains(&position) == false {
//                     println!("(ง •̀_•́)ง '{position}' is not a valid input, type only a number between or equal 1 and 9.");
//                     continue;
//                 }

//                 break position;
//             }

//             println!("(ง •̀_•́)ง '{position}' is not a valid input, type only a number between or equal 1 and 9.");
//         }
//     }
// }