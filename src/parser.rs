extern crate rustc_serialize;
use self::rustc_serialize::json::Json;

pub fn parse_message(input : String) -> String {
    let data = Json::from_str(&input); 
    match data{
      Ok(x) => String::from("String"),
      Err(_) => String::from("")
   }
}
