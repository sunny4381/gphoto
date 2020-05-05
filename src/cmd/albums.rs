use hyper::header::{UserAgent, Authorization};

use hyper;
use hyper::Client;
use serde_json;

use url::form_urlencoded;

use super::Args;
use goauth::{client, USER_AGENT};
use config::Config;
use error::Error;

const ALBUM_API_URL: &'static str = "https://photoslibrary.googleapis.com/v1/albums";

fn puts_albums_and_next(client: &Client, access_token: &str, url: &str) -> Result<(), Error> {
    let res = client.get(url)
        .header(Authorization(format!("Bearer {}", access_token)))
        .header(UserAgent(USER_AGENT.to_owned()))
        .send()?;

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HttpError(res.status));
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
        let next_url_params: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("pageToken", next_token)
            .finish();

        let next_url = format!("{}?{}", ALBUM_API_URL, next_url_params);

        return puts_albums_and_next(client, access_token, &next_url);
    }

    Ok(())
}

pub fn execute_albums(_args: &Args) -> Result<(), Error> {
    let config = Config::load("default")?;
    let access_token = config.access_token;

    let client = client()?;
    puts_albums_and_next(&client, &access_token, ALBUM_API_URL)
}
