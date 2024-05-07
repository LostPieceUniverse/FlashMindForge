use std::io::stdin;
use::std::fs;
use std::error::Error;
use serde::{Serialize, Deserialize};
//use rand::seq::SliceRandom;
// This means that there must be a frog.rs in the src/ folder
// mod frog;
use std::io;
use std::thread;
use std::time::Duration;
use ratatui::backend::CrosstermBackend;
use ratatui::{
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

//-----------------------------------variables----------------------------------------------------------
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
//--------------enum-------------
enum CardType { 
    Basic, Typing,
}

impl PartialEq for CardType{
    fn eq(&self, other: &Self) -> bool {
        match (self, other){
            (CardType::Basic, CardType::Basic) => true,
            (CardType::Typing, CardType::Typing) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize)]
//-----------card struct---------
struct Card {
    card_type: CardType,
    front: String,
    back: String,
    card_no: i32,
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

#[derive(Serialize, Deserialize)]
//--------add_deck struct--------
struct Deck {
    deck_name: String, 
    vec_of_cards: Vec<Card>,
}

impl Clone for Deck {
    fn clone(&self) -> Self {
        Deck {
            deck_name: self.deck_name.clone(),
            vec_of_cards: self.vec_of_cards.iter().map(|card| card.clone()).collect(),
        }
    }
}

impl Card { 
    fn new(card_type: CardType, front: String, back: String, card_no: i32) -> Card { 
        return Card {
            card_type,
            front,
            back,
            card_no
        };
    }
}

use std::fs::OpenOptions;
use std::io::Write;
//-----------------------------------------------help functions-----------------------------------------------
fn get_deck(deck_name: &str) -> Vec<Card>{

    let decks: Deck = deserialize_deck(&deck_name).unwrap();
    let vec_of_cards: Vec<Card> = decks.vec_of_cards;
    return vec_of_cards
}

fn list_decks(){
    let path = fs::read_dir("./").unwrap();

    for path in path {
        
        println!("{}", path.unwrap().path().display())
    }
}
//-------------------------------------------serialize deserialize-------------------------------------------
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

fn deserialize_deck(deck_name: &str) -> Result<Deck, Box<dyn Error>>{

    let path = "/home/haru/dev/rust/myownanki/";
    let mut filename = deck_name.trim().to_string();
    filename = format!("{}.json", filename);

    let complete_path = format!("{}{}",path, filename);

    let contents: String = fs::read_to_string(complete_path).expect("Should have been able to read the file");

    let decks: Deck  = serde_json::from_str(&contents).unwrap();
    Ok(decks)
}

//------------------------------------------------------functions----------------------------------------------
fn home() -> String{
    println!("creat deck (1)");
    println!("Study deck (2)");
    println!("Add cards(3)");
    println!("Exit (0)");

    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();
    return input;
    
}
fn add_new_cards(){
    let mut deck_name = String::new();

    println!("Which deck would you like to add cards to? ");
    stdin().read_line(&mut deck_name).unwrap();
    
    let vec_of_cards: Vec<Card> = get_deck(&deck_name);

    for item in vec_of_cards{
        println!("{}", item.front);
    }
    
}
fn add_new_deck(){
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

fn study(){
    let mut vec_of_hard: Vec<Card> = Vec::new();
    let mut vec_of_middle: Vec<Card> = Vec::new();
    let mut vec_of_easy: Vec<Card> = Vec::new();

    let mut deck_name = String::new();
    println!("Which deck would you like to study? ");
    list_decks();
    stdin().read_line(&mut deck_name).unwrap();
    
    //deserialize deck
    let vec_of_cards: Vec<Card> = get_deck(&deck_name); //get vector of cards
    
    for item in vec_of_cards{
        let mut enter = String::new();
        if item.card_type == CardType::Basic{
            println!("********************");
            println!("Front: {}", item.front);
            println!("--------------------");
            stdin().read_line(&mut enter).unwrap();
            print!("Back: {}", item.back);
            println!("********************\n");
            let mut input = String::new();
            println!("Easy (1), Middle (2), Hard (3)");
            stdin().read_line(&mut input).unwrap();
            match input.as_str(){
                "1\n" => vec_of_easy.push(item),
                "2\n" => vec_of_middle.push(item),
                "3\n"=> vec_of_hard.push(item),
                _ => panic!("smth went wrong..."),
            };
        }
        else if item.card_type == CardType::Typing{
            println!("********************");
            println!("Front: {}", item.front);
            println!("--------------------");
            stdin().read_line(&mut enter).unwrap();
            if enter == item.back{
                println!("\x1B[32mCorrect!\x1B[0m");
                vec_of_easy.push(item);
            }
            else{
                println!("\x1b[31mCorrection: {}\x1b[0m", item.back);
                vec_of_middle.push(item);
            }
            println!("********************\n");

        }
    };
    let mut middle_temp: Vec<Card> = vec_of_middle.clone();
    let mut hard_temp: Vec<Card> = vec_of_hard.clone();
    if hard_temp.is_empty(){
        println!("is empty");
    }
    while !middle_temp.is_empty() || !hard_temp.is_empty(){ 
        middle_temp  = vec_of_middle.clone();
        hard_temp = vec_of_hard.clone();
        println!("Currently {} easy cards, {} middle cards and {} hard cards", vec_of_easy.len(), vec_of_middle.len(), vec_of_hard.len());
        println!("press enter to continue or 0 to exit.");
        let mut temp = String::new();
        stdin().read_line(&mut temp).unwrap();
        if temp == "0\n"{
            break;
        }
        for item in middle_temp.iter(){

            let mut enter = String::new();
            if item.card_type == CardType::Basic{
                println!("********************");
                println!("Front: {}", item.front);
                println!("--------------------");
                stdin().read_line(&mut enter).unwrap();
                print!("Back: {}", item.back);
                println!("********************\n");
                let mut input = String::new();
                println!("Easy (1), Middle (2), Hard (3)");
                stdin().read_line(&mut input).unwrap();
                match input.as_str(){
                    "1\n" => vec_of_easy.push(item.clone()),
                    "2\n" => vec_of_middle.push(item.clone()),
                    "3\n"=> vec_of_hard.push(item.clone()),
                    _ => vec_of_middle.push(item.clone()),
                };
            }
            else if item.card_type == CardType::Typing{
                println!("********************");
                println!("Front: {}", item.front);
                println!("--------------------");
                stdin().read_line(&mut enter).unwrap();
                if enter == item.back{
                    println!("\x1B[32mCorrect!\x1B[0m");
                    vec_of_easy.push(item.clone());
                }
                else{
                    println!("\x1b[31mCorrection: {}\x1b[0m", item.back);
                    vec_of_middle.push(item.clone());
                }
                println!("********************\n");
            }
        };

        for item in hard_temp.iter(){

            let mut enter = String::new();
            if item.card_type == CardType::Basic{
                println!("********************");
                println!("Front: {}", item.front);
                println!("--------------------");
                stdin().read_line(&mut enter).unwrap();
                print!("Back: {}", item.back);
                println!("********************\n");
                let mut input = String::new();
                println!("Easy (1), Middle (2), Hard (3)");
                stdin().read_line(&mut input).unwrap();
                match input.as_str(){
                    "1\n" => vec_of_middle.push(item.clone()),
                    "2\n" => vec_of_middle.push(item.clone()),
                    _ => continue,
                };
            }
            else if item.card_type == CardType::Typing{
                println!("********************");
                println!("Front: {}", item.front);
                println!("--------------------");
                stdin().read_line(&mut enter).unwrap();
                if enter == item.back{
                    println!("\x1B[32mCorrect!\x1B[0m");
                    vec_of_middle.push(item.clone());
                }
                else{
                    println!("\x1b[31mCorrection: {}\x1b[0m", item.back);
                }
                println!("********************\n");
            }

        };
    };
}

//-----------------------------------------------------main------------------------------------------------
fn main() {
    // let mut == var 
    // let == const
    loop{
        let input = home();//get return of function home()
    
        if input == "0\n"{
            println!("okey bye");
            break;
        }

        match input.as_str(){
            "1\n" => {add_new_deck();},
            "3\n" => {add_new_cards();}
            "2\n"=> {study();},
            _ => panic!("smth went wrong..."),
        };
        println!("Done!");
    }
}

