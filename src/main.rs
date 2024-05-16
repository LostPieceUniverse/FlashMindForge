use std::io::stdin;

use card::Card;
use crate::card::*;
use crate::deck::*;

//modules
mod deck;
mod card;

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

