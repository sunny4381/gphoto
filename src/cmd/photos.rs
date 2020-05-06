use std::collections::HashMap;

use clap::ArgMatches;
use reqwest;
use reqwest::blocking::Client;
use serde_json;

use goauth::USER_AGENT;
use config::Config;
use error::Error;

const MEDIA_ITEM_LIST_URL: &'static str = "https://photoslibrary.googleapis.com/v1/mediaItems";
const MEDIA_ITEM_SEARCH_URL: &'static str = "https://photoslibrary.googleapis.com/v1/mediaItems:search";

fn puts_media_items(items: &Vec<serde_json::Value>) {
    for item in items {
        let id = item["id"].as_str();
        let description = item["description"].as_str();
        let mime_type = item["mimeType"].as_str();
        let timestamp = item["mediaMetadata"]["creationTime"].as_str();
        let width = item["mediaMetadata"]["width"].as_str();
        let height = item["mediaMetadata"]["height"].as_str();
        let filename = item["filename"].as_str();
        println!("{}\t{}\t{}\t{}\t{}x{}\t{}", id.unwrap_or(""), description.unwrap_or(""), mime_type.unwrap_or(""), timestamp.unwrap_or(""), width.unwrap_or(""), height.unwrap_or(""), filename.unwrap_or(""));
    }
}

fn list_all_library_contents(client: &Client, access_token: &str, page_token: &Option<&str>) -> Result<(), Error> {
    let mut req = client.get(MEDIA_ITEM_LIST_URL)
        .bearer_auth(access_token)
        .header(reqwest::header::USER_AGENT, USER_AGENT);
    if let Some(page_token) = page_token {
        req = req.query(&[("pageToken", page_token)]);
    }
    let res = req.send()?;

    if !res.status().is_success() {
        return Err(Error::from(res));
    }

    let response: serde_json::Value = serde_json::from_reader(res)?;
    match response["mediaItems"].as_array() {
        Some(items) => puts_media_items(items),
        _ => {
            if page_token.is_none() {
                println!("no photos were found");
            }
        }
    }

    if let Some(next_token) = response["nextPageToken"].as_str() {
        return list_all_library_contents(client, access_token, &Some(next_token));
    }

    Ok(())
}

fn list_all_album_contents(client: &Client, access_token: &str, album_id: &str, page_token: &Option<&str>) -> Result<(), Error> {
    let mut params = HashMap::new();
    params.insert("albumId", album_id);
    if let Some(page_token) = page_token {
        params.insert("pageToken", page_token);
    }

    let res = client.post(MEDIA_ITEM_SEARCH_URL)
        .bearer_auth(access_token)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .form(&params)
        .send()?;

    if !res.status().is_success() {
        return Err(Error::from(res));
    }

    let response: serde_json::Value = serde_json::from_reader(res)?;
    match response["mediaItems"].as_array() {
        Some(items) => puts_media_items(items),
        _ => {
            if page_token.is_none() {
                println!("no photos were found");
            }
        }
    }

    if let Some(next_token) = response["nextPageToken"].as_str() {
        return list_all_album_contents(client, access_token, album_id, &Some(next_token));
    }

    Ok(())
}

pub fn execute_photos(args: &ArgMatches) -> Result<(), Error> {
    let config = Config::load("default")?;
    let client = Client::new();

    match args.value_of("album_id") {
        Some(album_id) => list_all_album_contents(&client, &config.access_token, album_id, &None),
        _ => list_all_library_contents(&client, &config.access_token, &None)
    }
}
