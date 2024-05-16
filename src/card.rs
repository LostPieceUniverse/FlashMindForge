use serde::{Serialize, Deserialize};
use std::io::stdin;

use crate::get_deck;

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

impl Card { 
    pub fn new(card_type: CardType, front: String, back: String, card_no: i32) -> Card { 
        return Card {
            card_type,
            front,
            back,
            card_no
        };
    }
}


//-----------functions---------
pub fn add_new_cards(){
    let mut deck_name = String::new();

    println!("Which deck would you like to add cards to? ");
    stdin().read_line(&mut deck_name).unwrap();
    
    let vec_of_cards: Vec<Card> = get_deck(&deck_name);

    for item in vec_of_cards{
        println!("{}", item.front);
    }
    
}

