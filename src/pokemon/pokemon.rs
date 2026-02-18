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
            let iv = input();

            let iv_int = iv.split(",").map(|x| {
                
                let m = x.trim().parse::<u16>().unwrap_or_else(|error| {
                    println!("Please enter numbers for your IV's, setting default to 31");
                    return 31
                });
                println!("{}", m);
                m
            });
            self.IV = iv_int.collect();
            break;
        }
        
    }

    fn EV(&mut self) {
        println!("Please enter your EV's in order of HP to speed seperated by comma\n IE, 252, 4, 252, 0, 0, 0");
        println!("Reminder that your EV's max at 252 for a single stat and you can only invest 508 in total")
    }
}