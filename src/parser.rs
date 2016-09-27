extern crate rustc_serialize;
use self::rustc_serialize::json::{Json, Object};
use std::collections::BTreeMap;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Message {
    PlayerNameRequest { player_number: String, version: u64},
    ValidMessage,
    InvalidMessage
}


pub fn parse_message(input : String) -> Message{
    Json::from_str(&input).map_err(|x| ()).and_then(|j| parse_json(j)).unwrap_or(Message::InvalidMessage) 
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

   #[test]
   fn invalid_json_returns_empty_string() {
        assert_eq!(parse_message(String::from("{")), super::Message::InvalidMessage) 
   }

   #[test]
   fn player_name_reply_message_is_handled() {
        let json = String::from("{\"type\": \"player-name-request\", \"player_number\": \"player2\", \"version\": 1}");
        assert_eq!(parse_message(json), super::Message::PlayerNameRequest{ player_number: String::from("player2"), version: 1}); 
   }
}
