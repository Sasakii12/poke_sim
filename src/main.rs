use std::io;

use crate::pokemon::types::{Nature, Types};

pub mod prompts;
mod pokemon;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
    .expect("Error taking input");

    buf
}


fn main() {
    // Test object
    let mut meowscarada = pokemon::pokemon::Pokemon::new(String::from("Meowscarada"),
    (Types::Grass, Types::Dark), 50,"Protean".to_string(), 76, 110, 70,
    81, 70, 123, Nature::Bold,[].to_vec(), [].to_vec(), "Leftovers".to_string());
    meowscarada.IV();

    loop {
        println!("What would you like to do?");
        println!("[1] Battle,\n[2] Teambuild,\n[3] Quit");
        let n = input();
        match n.as_str().trim() {
            "1" => panic!("Not implemented"),
            "2" => prompts::team_file_create(),
            "3" => break,
            _ => {
                println!("Please enter a number as your option");
                continue;}
        }
    }
}

