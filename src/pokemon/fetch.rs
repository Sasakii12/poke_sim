use reqwest::{Error};
use serde::Deserialize;
use serde_json::Value;

use crate::pokemon::pokemon::PokemonBaseStats;

#[derive(Deserialize, Debug)]
pub struct PokemonJson {
    abilities: String,
    moves: String,
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

    let moves = poke["moves"].as_array().map(|arr| {
        arr.iter().filter_map(|mv| mv["move"]["name"].as_str()).collect::<Vec<_>>().join(", ")
    }).unwrap_or_default();

    let moves = poke["moves"].as_array().map(|arr| {
        arr.iter().filter_map(|mv| mv["move"]["name"].as_str()).collect::<Vec<_>>().join(", ")
    }).unwrap_or_default();

    let stats = poke["stats"].as_array().map(|arr| {
        arr.iter().filter_map(|st| Some((st["stat"]["name"].as_str(),st["base_stat"].to_string().parse::<u16>()))).collect::<Vec<_>>()
    }).unwrap_or_default();
    let s = stats.iter().map(|x| {
        (x.0.unwrap().to_string(), x.1.as_ref().unwrap().to_owned())
    }).collect::<Vec<(String,u16)>>();
    println!("{:?}", s);
    todo!("")

}