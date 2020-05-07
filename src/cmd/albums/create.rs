use clap::ArgMatches;
use reqwest;
use serde_json::json;

use crate::goauth::USER_AGENT;
use crate::config::Config;
use crate::error::Error;

const ALBUM_API_URL: &'static str = "https://photoslibrary.googleapis.com/v1/albums";

pub fn execute_albums_create(args: &ArgMatches) -> Result<(), Error> {
    let name = args.value_of("name").unwrap_or_else(|| panic!("specify album name"));

    let config = Config::load("default")?;
    let access_token = config.access_token;

    let client = reqwest::blocking::Client::new();

    let request_body = json!({
        "album": {
            "title": name,
        }
    });
    let request_json = request_body.to_string();
    let req = client.post(ALBUM_API_URL)
        .bearer_auth(access_token)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_json);

    let res = req.send()?;
    if !res.status().is_success() {
        return Err(Error::from(res));
    }

    let album_json: serde_json::Value = serde_json::from_reader(res)?;
    let id = album_json["id"].as_str().unwrap();
    let product_url = album_json["productUrl"].as_str().unwrap();
    // album_json["title"].as_str();
    // album_json["isWriteable"].as_str();

    println!("created {}({})", id, product_url);

    Ok(())
}
