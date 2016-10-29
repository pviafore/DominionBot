pub enum Card { 
    Copper,
    Silver,
    Gold,
    Estate,
    Duchy,
    Province,
    Curse,
    Cellar,
    Market,
    Militia,
    Mine,
    Moat,
    Remodel,
    Smithy,
    Village,
    Woodcutter,
    Workshop
}

pub fn to_card(str: String) -> Option<Card> {
   match str {
     "copper" => Some(Card::Copper),
     "silver" => Some(Card::Silver),
     "gold" => Some(Card::Gold),
     "estate" => Some(Card::Estate),
     "duchy" => Some(Card::Duchy),
     "province" => Some(Card::Province),
     "curse" => Some(Card::Curse),
     "cellar" => Some(Card::Cellar),
     "market" => Some(Card::Market),
     "militia" => Some(Card::Militia),
     "mine" => Some(Card::Mine),
     "moat" => Some(Card::Moat),
     "remodel" => Some(Card::Remodel),
     "smithy" => Some(Card::Smithy),
     "village" => Some(Card::Village),
     "woodcutter" => Some(Card::Woodcutter),
     "workshop" => Some(Card::Workshop),
     _ => None
  }
}
