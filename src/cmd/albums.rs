use clap::ArgMatches;
use reqwest::blocking::Client;
use serde_json;

use goauth::USER_AGENT;
use config::Config;
use error::Error;

const ALBUM_API_URL: &'static str = "https://photoslibrary.googleapis.com/v1/albums";

fn puts_albums_and_next(client: &Client, access_token: &str, page_token: &Option<&str>) -> Result<(), Error> {
    let mut req = client.get(ALBUM_API_URL)
        .bearer_auth(access_token)
        .header(reqwest::header::USER_AGENT, USER_AGENT);
    if let Some(page_token) = page_token {
        req = req.query(&[("pageToken", page_token)]);
    }
    let res = req.send()?;

    if !res.status().is_success() {
        return Err(Error::from(res));
    }

    let albums_json: serde_json::Value = serde_json::from_reader(res)?;
    match albums_json["albums"].as_array() {
        Some(entries) => {
            for entry in entries {
                let id = entry["id"].as_str();
                let title = entry["title"].as_str();
                let url = entry["productUrl"].as_str();
                let count = entry["mediaItemsCount"].as_str();
                match (id, title) {
                    (Some(id), Some(title)) => println!("{}\t{}\t{}\t{}", id, title, url.unwrap_or(""), count.unwrap_or("")),
                    _ => ()
                };
            };
        }
        _ => { }
    }

    if let Some(next_token) = albums_json["nextPageToken"].as_str() {
        return puts_albums_and_next(client, access_token, &Some(next_token));
    }

    Ok(())
}

pub fn execute_albums(_args: &ArgMatches) -> Result<(), Error> {
    let config = Config::load("default")?;
    let access_token = config.access_token;

    let client = reqwest::blocking::Client::new();
    puts_albums_and_next(&client, &access_token, &None)
}
