pub struct PlayerNameReply {
    player_number: String,
    name: String,
    version: u64
}

pub enum AttackReply {

    AttackReply { discard: Vec<String> },
    Reaction
} 

pub enum ActionCard {

   Cellar,
   Market,
   Mine,
   Militia,
   Moat,
   Remodel,
   Smithy,
   Village,
   Woodcutter,
   Workshop

}

pub enum PlayReply {

   Action(ActionCard),
   Buy {played_treasures: Vec<String>, cards_to_buy: Vec<String>},
   Cleanup {top_discard: String}
}
