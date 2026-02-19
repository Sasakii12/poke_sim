use std::string::ParseError;

use crate::input;


// Used for constructing the full stats of a pokemon
pub struct PokemonBaseStats {
    name: String,
    types: (String, String),
    ability: String,
    hp: u16,
    attack: u16,
    defense: u16,
    spatk: u16,
    spdef: u16,
    speed: u16,
}

// The resulting pokemon when fully IV'd, EV'd, and item'd
pub struct Pokemon {
    name: String,
    types: (String, String),
    ability: String,
    hp: u16,
    attack: u16,
    defense: u16,
    spatk: u16,
    spdef: u16,
    speed: u16,
    nature: String,
    IV: Vec<u16>,
    EV: Vec<u16>,
    item: String,
}

impl Pokemon {
    pub fn new(name: String, types: (String, String), ability: String,
    hp: u16, attack: u16, defense: u16, spatk: u16, spdef: u16, speed: u16, nature: String,
    IV: Vec<u16>, EV: Vec<u16>, item: String) -> Pokemon 
    {
        Pokemon { name, types, ability, hp, attack, defense, spatk, spdef, speed, nature, IV, EV, item }
    }

    pub fn IV(&mut self) {
        loop {
            println!("Please enter your IV's in order of HP to speed seperated by comma\n IE, 31, 25, 20, 31, 25, 31");
            let iv = input().split(",").map(|x| x.trim().parse::<u16>()).collect::<Result<Vec<u16>, _>>();
            let iv = match iv {
                Ok(f) => f,
                Err(e) => continue,
            };

            self.IV = iv;
            break;
        }
    }
    pub fn EV(&mut self) {
        loop {
            println!("Please enter your EV's in order of HP to speed seperated by comma\n IE, 252, 4, 252, 0, 0, 0");
            println!("Reminder that your EV's max at 252 for a single stat and you can only invest 508 in total");
            let EV = input().split(",").map(|x| x.trim().parse::<u16>()).collect::<Result<Vec<u16>, _>>();
            let EV = match EV {
                Ok(f) => f,
                Err(e) => {
                    continue;
                }
            };
            self.EV = EV;
            break;
        }
        
    }
}