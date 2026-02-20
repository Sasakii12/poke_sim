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

    fn stat_eq(stat_name: String, stat: u16, IV: u16, EV: u16, level: u16, nature: Nature) -> u16 {
        let nature_stat_change = nature.eval_nature();
        if nature_stat_change.0 == stat_name {
            ((((2. * stat as f32 + IV as f32 + (EV as f32 / 4.)) * level as f32) / 100.).floor() + 5. * 1.1).floor() as u16
        } else if nature_stat_change.1 == stat_name {
            ((((2. * stat as f32 + IV as f32 + (EV as f32 / 4.)) * level as f32) / 100.).floor() + 5. * 0.9).floor() as u16
        } else {
            ((((2. * stat as f32 + IV as f32 + (EV as f32 / 4.)) * level as f32) / 100.).floor() + 5.).floor() as u16
        }
    }

    pub fn eval(&mut self) {
        // By default level should be 100
        self.level = 100;

        self.hp = Pokemon::hp_eq(self.hp, self.IV[0], self.EV[0], self.level);
        self.attack = Pokemon::stat_eq(String::from("Attack"), self.attack, self.IV[1], self.EV[1], self.level, self.nature);
        self.defense = Pokemon::stat_eq(String::from("Defense"), self.defense, self.IV[2], self.EV[2], self.level, self.nature);
        self.spatk = Pokemon::stat_eq(String::from("Spatk"), self.spatk, self.IV[3], self.EV[3], self.level, self.nature);
        self.spdef = Pokemon::stat_eq(String::from("Spdef"), self.spdef, self.IV[4], self.EV[4], self.level, self.nature);
        self.speed = Pokemon::stat_eq(String::from("Speed"), self.speed, self.IV[5], self.EV[5], self.level, self.nature);
    }
}