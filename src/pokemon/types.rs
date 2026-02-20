 use std::collections::HashMap;
 
 pub enum Types {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy
}

#[derive(Copy, Clone)]
pub enum Nature {
    // Neutral natures
    Hardy,
    Docile,
    Bashful,
    Quirky,
    Serious, 

    // Attack natures
    Lonely,
    Adamant,
    Naughty,
    Brave,

    // Defense natures
    Bold,
    Impish,
    Lax,
    Relaxed,

    // Spatk natures
    Modest,
    Mild,
    Rash,
    Quiet,

    // Spdef natures
    Calm,
    Gentle,
    Careful,
    Sassy,

    // Speed natures
    Timid,
    Hasty,
    Jolly,
    Naive

}

impl Nature {
    pub fn eval_nature(self) -> (String, String) {
        let mut n1 = String::from("");
        let mut n2 = String::from("");
        match self {
            // Natures are always in the order of increase and decrease
            // For example Lonely is a "+attack" and "-defense" nature

            // +Attack natures
            Nature::Lonely => (String::from("Attack"), String::from("Defense")),
            Nature::Adamant => (String::from("Attack"), String::from("Spatk")),
            Nature::Naughty => (String::from("Attack"), String::from("Spdef")),
            Nature::Brave => (String::from("Attack"), String::from("Speed")),

            // Defense natures
            Nature::Bold => (String::from("Defense"), String::from("Attack")),
            Nature::Impish => (String::from("Defense"), String::from("Spatk")),
            Nature::Lax => (String::from("Defense"), String::from("Spdef")),
            Nature::Relaxed => (String::from("Defense"), String::from("Speed")),

            // Special attack natures
            Nature::Modest => (String::from("Spatk"), String::from("Attack")),
            Nature::Mild => (String::from("Spatk"), String::from("Defense")),
            Nature::Rash => (String::from("Spatk"), String::from("Spdef")),
            Nature::Quiet => (String::from("Spatk"), String::from("Speed")),

            // Special defense natures
            Nature::Calm => (String::from("Spdef"), String::from("Attack")),
            Nature::Gentle => (String::from("Spdef"), String::from("Defense")),
            Nature::Careful => (String::from("Spdef"), String::from("Spatk")),
            Nature::Sassy => (String::from("Spdef"), String::from("Speed")),

            // Speed natures
            Nature::Timid => (String::from("Speed"), String::from("Attack")),
            Nature::Hasty => (String::from("Speed"), String::from("Defense")),
            Nature::Jolly => (String::from("Speed"), String::from("Spatk")),
            Nature::Naive => (String::from("Speed"), String::from("Spdef")),
            
            // Neutral natures (do nothing)
            _ => (String::from(""), String::from(""))
        }
    }
}