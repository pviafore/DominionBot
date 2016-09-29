extern crate rustc_serialize;
use self::rustc_serialize::json::{Json, Object};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Message {
    PlayerNameRequest { player_number: String, version: u64},
    PlayTurnRequest { actions: u64, buys: u64, extra_money: u64, hand: Vec<String>, cards_played: Vec<String>, cards_gained: Vec<String> },
    GameInfoMessage,
    SupplyInfoMessage,
    PlayerGainedMessage,
    PlayerPlayedMessage,
    PlayerTrashedMessage,
    PlayerTopDiscardMessage,
    PlayerRevealedMessage,
    PlayerShuffledMessage,
    AttackRequest { discard: u64, options: Vec<String> },
    GameEndMessage,
    InvalidMessage
}

pub fn parse_message(input : String) -> Message{
    Json::from_str(&input).map_err(|_| ()).and_then(parse_json).unwrap_or(Message::InvalidMessage) 
}

fn parse_json(json: Json) -> Result<Message, ()> {
   let json_obj = try!(json.as_object().ok_or(()));
   let message_type = try!(get_string(json_obj, "type"));
   match message_type.as_str() {
      "player-name-request" => create_player_name_message(json_obj),
      "play-turn" => create_play_turn_message(json_obj),
      "attack-request" => create_attack_request_message(json_obj),
      "game-info" => Ok(Message::GameInfoMessage),
      "supply-info" => Ok(Message::SupplyInfoMessage),
      "player-gained" => Ok(Message::PlayerGainedMessage),
      "player-played" => Ok(Message::PlayerPlayedMessage),
      "player-trashed" => Ok(Message::PlayerTrashedMessage),
      "player-top-discard" => Ok(Message::PlayerTopDiscardMessage),
      "player-reveal" => Ok(Message::PlayerRevealedMessage),
      "player-shuffled" => Ok(Message::PlayerShuffledMessage),
      "game-end" => Ok(Message::GameEndMessage),
      _ => Err(())
   }
} 

fn get_string(json_obj : &Object, key: &str) -> Result<String, ()> {
  json_obj.get(key).and_then(|s: &Json| s.as_string().map(String::from)).ok_or(())
}

fn get_string_list(json_obj : &Object, key: &str) -> Result<Vec<String>, ()> {
  let array = json_obj.get(key).and_then(|s: &Json| s.as_array().cloned());
  match array { 
     Some(arr) => convert_array_to_string(arr),
     None => Err(()) 
  }
}

fn convert_array_to_string(array: Vec<Json>) -> Result<Vec<String>, ()> {
   array.iter().map(|s| s.as_string().map(String::from).ok_or(())).collect()
}

fn get_num(json_obj : &Object, key: &str) -> Result<u64, ()> {
  json_obj.get(key).and_then(|i: &Json| i.as_u64()).ok_or(())
}

fn create_player_name_message(data : &Object) -> Result<Message, ()> {
   let player_number = try!(get_string(data, "player_number"));
   let version = try!(get_num(data, "version"));
   Ok(Message::PlayerNameRequest { player_number: player_number, version: version})
}

fn create_play_turn_message(data: &Object) -> Result<Message, ()> {
   let actions = try!(get_num(data, "actions"));
   let buys = try!(get_num(data, "buys"));
   let extra_money = try!(get_num(data, "extra_money"));
   let hand = try!(get_string_list(data, "hand"));
   let cards_played = try!(get_string_list(data, "cards_played"));
   let cards_gained = try!(get_string_list(data, "cards_gained"));
   Ok(Message::PlayTurnRequest { actions: actions, buys: buys, extra_money: extra_money, hand: hand, cards_played: cards_played, cards_gained: cards_gained})
}

fn create_attack_request_message(data: &Object) -> Result<Message, ()> {
   let discard = try!(get_num(data, "discard"));
   let options = try!(get_string_list(data, "options"));
   Ok(Message::AttackRequest { discard: discard, options: options})
}

#[cfg(test)]
mod tests {
   use super::parse_message;
   fn assert_invalid_message(s: &str) {
        assert_eq!(parse_message(s.to_string()), super::Message::InvalidMessage) 
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
        assert_invalid_message("{\"type\": \"attack-request\", \"discard\": 2}"); 
        assert_invalid_message("{\"type\": \"attack-request\",  \"options\": []}");

   }

   #[test]
   fn catchall_test_for_unsupported_types() {
      assert_eq!(parse_message("{\"type\": \"game-info\"}".to_string()), super::Message::GameInfoMessage);
      assert_eq!(parse_message("{\"type\": \"supply-info\"}".to_string()), super::Message::SupplyInfoMessage);
      assert_eq!(parse_message("{\"type\": \"player-gained\"}".to_string()), super::Message::PlayerGainedMessage);
      assert_eq!(parse_message("{\"type\": \"player-played\"}".to_string()), super::Message::PlayerPlayedMessage);
      assert_eq!(parse_message("{\"type\": \"player-trashed\"}".to_string()), super::Message::PlayerTrashedMessage);
      assert_eq!(parse_message("{\"type\": \"player-top-discard\"}".to_string()), super::Message::PlayerTopDiscardMessage);
      assert_eq!(parse_message("{\"type\": \"player-reveal\"}".to_string()), super::Message::PlayerRevealedMessage);
      assert_eq!(parse_message("{\"type\": \"player-shuffled\"}".to_string()), super::Message::PlayerShuffledMessage);
      assert_eq!(parse_message("{\"type\": \"game-end\"}".to_string()), super::Message::GameEndMessage);
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

  #[test]
  fn attack_request_is_handled() { 
      let json = "{\"type\": \"attack-request\", \"discard\": 2, \"options\": [\"copper\"]}".to_string();
      assert_eq!(parse_message(json), super::Message::AttackRequest{ discard: 2, options: vec!["copper".to_string()] });
  }
}
