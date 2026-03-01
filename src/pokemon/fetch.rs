use reqwest::{Error};
use serde::Deserialize;
use serde_json::Value;

use crate::pokemon::pokemon::PokemonBaseStats;

#[derive(Deserialize, Debug)]
pub struct PokemonJson {
    abilities: String,
    moves: String,
    species: String,
    stats: String,
    types: String,
}


pub async fn fetch_pokemon(pokemon: &str, url: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let full_url = url.to_owned() + pokemon;
    let req = client.get(full_url).send().await?.text().await?;
    Ok(req)
}

pub async fn parse_pokemon(poke_json: String) -> Result<PokemonJson, serde_json::Error> {
    let poke: Value = serde_json::from_str(&poke_json)?;

    let abilities = poke["abilities"].as_array().map(|arr| {
        arr.iter().filter_map(|item| item["ability"]["name"].as_str()).collect::<Vec<_>>().join(", ")
    }).unwrap_or_default();
    println!("{}", abilities);
    todo!("")

}