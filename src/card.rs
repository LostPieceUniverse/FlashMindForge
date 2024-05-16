use serde::{Serialize, Deserialize};

//-----------card struct---------
#[derive(Serialize, Deserialize)]
pub struct Card {
    pub card_type: CardType,
    pub front: String,
    pub back: String,
    pub card_no: i32,
}

//-----------enum---------
#[derive(Clone, Serialize, Deserialize)]
pub enum CardType { 
    Basic, 
    Typing,
}

//-----------implementations---------
impl PartialEq for CardType{
    fn eq(&self, other: &Self) -> bool {
        match (self, other){
            (CardType::Basic, CardType::Basic) => true,
            (CardType::Typing, CardType::Typing) => true,
            _ => false,
        }
    }
}

impl Clone for Card {
    fn clone(&self) -> Self {
        Card {
            card_type: self.card_type.clone(),
            front: self.front.clone(),
            back: self.back.clone(),
            card_no: self.card_no,
        }
    }
}
