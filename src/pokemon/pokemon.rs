use std::string::ParseError;
use crate::{input, pokemon::types::{Types, Nature}};


// Used for constructing the full stats of a pokemon
pub struct PokemonBaseStats {
    name: String,
    types: (Types, Types),
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
    types: (Types, Types),
    level: u16,
    ability: String,
    hp: u16,
    attack: u16,
    defense: u16,
    spatk: u16,
    spdef: u16,
    speed: u16,
    nature: Nature,
    IV: Vec<u16>,
    EV: Vec<u16>,
    item: String,
}

impl PokemonBaseStats {
    pub fn new() {}
}

impl Pokemon {
    pub fn new(name: String, types: (Types, Types), level: u16, ability: String,
    hp: u16, attack: u16, defense: u16, spatk: u16, spdef: u16, speed: u16, nature: Nature,
    IV: Vec<u16>, EV: Vec<u16>, item: String) -> Pokemon 
    {
        Pokemon { name, types, level, ability, hp, attack, defense, spatk, spdef, speed, nature, IV, EV, item }
    }

    pub fn IV(&mut self) {
        loop {
            println!("Please enter your IV's in order of HP to speed seperated by comma\n IE, 31, 25, 20, 31, 25, 31");
            let iv = input().split(",").map(|x| x.trim().parse::<u16>()).collect::<Result<Vec<u16>, _>>();
            let iv = match iv {
                Ok(f) => f,
                Err(_) => continue,
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
                Err(_) => {
                    continue;
                }
            };
            self.EV = EV;
            break;
        }
        
    }

    fn hp_eq(hp: u16, IV: u16, EV: u16, level: u16) -> u16 {
        (((2. * hp as f32 + IV as f32 + (EV as f32 / 4.)) * level as f32) / 100.).floor() as u16 + level + 10
    }

    fn stat_eq(stat: u16, IV: u16, EV: u16, level: u16, nature: String) {

    }

    pub fn eval(&mut self) {
        let level = self.level;
    }
}