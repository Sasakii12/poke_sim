use reqwest::{Error};

pub async fn fetch_pokemon(pokemon: &str, url: &str) -> Result<String, Error> {
    let full_url = url.to_owned() + pokemon;
    let req = reqwest::get(full_url).await?.text().await?;
    Ok(req)
}