use std::io;

use crate::pokemon::{fetch, pokemon::PokemonBaseStats, types::{Nature, Types}};

pub mod prompts;
mod pokemon;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
    .expect("Error taking input");

    buf
}


#[tokio::main]
async fn main() {
    // Test object
    // let meowscarada_base = PokemonBaseStats::new(String::from("Meowscarada"),
    // (Types::Grass, Types::Dark),"Protean".to_string(), 76, 110, 70,
    // 81, 70, 123);
    // let mut meowscarada = pokemon::pokemon::Pokemon::new(meowscarada_base, 100, Nature::Adamant,[].to_vec(), [].to_vec(), "Leftovers".to_string());
    // meowscarada.display();
    // meowscarada.eval();
    // meowscarada.display();

    println!("{}", fetch::fetch_pokemon("Pikachu", "https://pokeapi.co/api/v2/pokemon/").await.unwrap());

    loop {
        println!("What would you like to do?");
        println!("[1] Battle,\n[2] Teambuild,\n[3] Quit");
        let n = input();
        match n.as_str().trim() {
            "1" => panic!("Not implemented"),
            "2" => poke_sim::team_file_create(),
            "3" => break,
            _ => {
                println!("Please enter a number as your option");
                continue;}
        }
    }
}

