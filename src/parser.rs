extern crate rustc_serialize;
use self::rustc_serialize::json::{Json, Object};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Message {
    PlayerNameRequest { player_number: String, version: u64},
    PlayTurnRequest { actions: u64, buys: u64, extra_money: u64, hand: Vec<String>, cards_played: Vec<String>, cards_gained: Vec<String> },
    InvalidMessage
}


pub fn parse_message(input : String) -> Message{
    Json::from_str(&input).map_err(|_| ()).and_then(|j| parse_json(j)).unwrap_or(Message::InvalidMessage) 
}

fn parse_json(json: Json) -> Result<Message, ()> {
   let json_obj = try!(json.as_object().ok_or(()));
   let message_type = try!(get_string(json_obj, "type"));
   match message_type.as_str() {
      "player-name-request" => create_player_name_message(json_obj),
      _ => Err(())
   }
} 

fn get_string(json_obj : &Object, key: &str) -> Result<String, ()> {
  json_obj.get(key).and_then(|s: &Json| s.as_string().map(|s| String::from(s))).ok_or(())
}


fn get_num(json_obj : &Object, key: &str) -> Result<u64, ()> {
  json_obj.get(key).and_then(|i: &Json| i.as_u64()).ok_or(())
}
fn create_player_name_message(data : &Object) -> Result<Message, ()> {
   let player_number = try!(get_string(data, "player_number"));
   let version = try!(get_num(data, "version"));
   Ok(Message::PlayerNameRequest { player_number: player_number, version: version})
}


#[cfg(test)]
mod tests {
   use super::parse_message;
   fn assert_invalid_message(s: &str) {
        assert_eq!(parse_message(String::from(s)), super::Message::InvalidMessage) 
   }
   #[test]
   fn invalid_json_returns_invalid_message() {
        assert_invalid_message("{"); 
        assert_invalid_message("{}");
        assert_invalid_message("{\"type\": \"unsupported\"}");
        assert_invalid_message("{\"type\": \"player-name-request\", \"version\": 1}");
        assert_invalid_message("{\"type\": \"player-name-request\", \"player_number\" : \"player1\"}");
        assert_invalid_message("{\"type\": \"player-name-request\", \"version\": \"1\", \"player_number\": \"player1\"}");
        assert_invalid_message("{\"type\": \"play-turn\"}");
        assert_invalid_message("{\"type\": \"play-turn\", \"buys\": 1, \"extra_money\": 1, \"hand\": [], \"cards_played\": [], \"cards_gained\": []}");
        assert_invalid_message("{\"type\": \"play-turn\", \"actions\": 1, \"extra_money\": 1, \"hand\": [], \"cards_played\": [], \"cards_gained\": []}");

        assert_invalid_message("{\"type\": \"play-turn\", \"actions\": 1, \"buys\": 1, \"hand\": [], \"cards_played\": [], \"cards_gained\": []}");
        assert_invalid_message("{\"type\": \"play-turn\", \"actions\": 1, \"buys\": 1, \"extra_money\": 1, \"cards_played\": [], \"cards_gained\": []}");
        assert_invalid_message("{\"type\": \"play-turn\", \"actions\": 1, \"buys\": 1, \"extra_money\": 1, \"hand\": [], \"cards_gained\": []}");
        assert_invalid_message("{\"type\": \"play-turn\", \"actions\": 1, \"buys\": 1, \"extra_money\": 1, \"hand\": [], \"cards_played\": []}");

   }

   #[test]
   fn player_name_reply_message_is_handled() {
        let json = String::from("{\"type\": \"player-name-request\", \"player_number\": \"player2\", \"version\": 1}");
        assert_eq!(parse_message(json), super::Message::PlayerNameRequest{ player_number: String::from("player2"), version: 1}); 
   }

   #[test]
   fn play_turn_request_is_handled() {
       let json = String::from("{\"type\": \"play-turn\", \"actions\": 1, \"buys\": 1, \"extra_money\": 1, \"hand\": [\"copper\"], \"cards_played\": [\"silver\"], \"cards_gained\": [\"gold\"]}");
       assert_eq!(parse_message(json), super::Message::PlayTurnRequest { actions: 1, buys: 1, extra_money: 1, hand: vec![String::from("copper")], cards_played: vec![String::from("silver")], cards_gained: vec![String::from("gold")]});
   }
}
