use serde::{Serialize, Deserialize};
use::std::fs;
use std::error::Error;
use std::io::stdin;
use std::fs::OpenOptions;
use std::io::Write;

use crate::Card;
use crate::card::CardType;

#[derive(Serialize, Deserialize)]
//--------deck struct--------
pub struct Deck {
    pub deck_name: String, 
    pub vec_of_cards: Vec<Card>,
}

//--------implementations--------
impl Clone for Deck {
    fn clone(&self) -> Self {
        Deck {
            deck_name: self.deck_name.clone(),
            vec_of_cards: self.vec_of_cards.iter().map(|card| card.clone()).collect(),
        }
    }
}

//--------functions--------
pub fn get_deck(deck_name: &str) -> Vec<Card>{

    let decks: Deck = deserialize_deck(&deck_name).unwrap();
    let vec_of_cards: Vec<Card> = decks.vec_of_cards;
    return vec_of_cards
}

pub fn list_decks(){
    let path = fs::read_dir("./").unwrap();

    for path in path {
        
        println!("{}", path.unwrap().path().display())
    }
}

pub fn add_new_deck(){
    //vector.vec_of_cards = Vec::new();//new vector(list)
    
    let mut vector: Vec<Card> = Vec::new();
    let mut numb_input = String::new();

    println!("How many cards will be added? ");
    stdin().read_line(&mut numb_input).unwrap();
    let numb_of_cards: i32 = numb_input.trim().parse().unwrap();

    for i in 0..numb_of_cards{
        
        let mut input = String::new();
        let mut front = String::new();
        let mut back = String::new();
        println!("Which card type?\r\nbasic (0) or typing (1)");
        stdin().read_line(&mut input).unwrap();

        let typeofcard = match input.as_str() {
            "0\n" => CardType::Basic, 
            "1\n" => CardType::Typing, 
            _ => panic!("hyelp"),
        };

        println!("Front: ");
        stdin().read_line(&mut front).unwrap();

        println!("Back: ");
        stdin().read_line(&mut back).unwrap();

        let card = Card::new(typeofcard, front.to_string(), back.to_string(), i);
        vector.push(card);
    }
    serialize_content(vector);
}

//--------deserialize serialize--------
pub fn deserialize_deck(deck_name: &str) -> Result<Deck, Box<dyn Error>>{

    let path = "/home/haru/dev/rust/myownanki/";
    let mut filename = deck_name.trim().to_string();
    filename = format!("{}.json", filename);

    let complete_path = format!("{}{}",path, filename);

    let contents: String = fs::read_to_string(complete_path).expect("Should have been able to read the file");

    let decks: Deck  = serde_json::from_str(&contents).unwrap();
    Ok(decks)
}

fn serialize_content(vector_contents: Vec<Card>){
    let mut deck_name = String::new();
    println!("Deck name: ");
    stdin().read_line(&mut deck_name).unwrap();

    deck_name = deck_name.trim().to_string();

    let filename = format!("{}.json", deck_name);

    let contents = Deck {
        deck_name: deck_name.to_string(),
        vec_of_cards: vector_contents,
    };
    
    let content = serde_json::to_string(&contents).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    writeln!(file,"{}", content).unwrap();
}
