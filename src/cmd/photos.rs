use clap::ArgMatches;

use hyper;
use hyper::header::{ContentType, UserAgent, Authorization};

use url::form_urlencoded;

use serde_json;

use goauth::{client, USER_AGENT};
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

fn list_all_library_contents(client: &hyper::Client, access_token: &str, token: Option<&str>) -> Result<(), Error> {
    let base_req = match token {
        Some(t) => {
            let next_url_params: String = form_urlencoded::Serializer::new(String::new())
                .append_pair("pageToken", t)
                .finish();
            let url = format!("{}?{}", MEDIA_ITEM_LIST_URL, next_url_params);
            client.get(&url)
        },
        _ => client.get(MEDIA_ITEM_LIST_URL)
    };
    let req = base_req.header(Authorization(format!("Bearer {}", access_token)))
        .header(UserAgent(USER_AGENT.to_owned()));
    let res = req.send()?;

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HttpError(res.status));
    }

    let response: serde_json::Value = serde_json::from_reader(res)?;
    match response["mediaItems"].as_array() {
        Some(items) => puts_media_items(items),
        _ => {
            if token.is_none() {
                println!("no photos were found");
            }
        }
    }

    if let Some(next_token) = response["nextPageToken"].as_str() {
        return list_all_library_contents(client, access_token, Some(next_token));
    }

    Ok(())
}

fn list_all_album_contents(client: &hyper::Client, access_token: &str, album_id: &str, token: Option<&str>) -> Result<(), Error> {
    let mut request_builder = form_urlencoded::Serializer::new(String::new());
    request_builder.append_pair("albumId", album_id);
    if let Some(t) = token {
        request_builder.append_pair("pageToken", t);
    }
    let request_body: String = request_builder.finish();

    let request = client.post(MEDIA_ITEM_SEARCH_URL)
        .header(Authorization(format!("Bearer {}", access_token)))
        .header(UserAgent(USER_AGENT.to_owned()))
        .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
        .body(&request_body);

    let res = request.send()?;

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HttpError(res.status));
    }

    let response: serde_json::Value = serde_json::from_reader(res)?;
    match response["mediaItems"].as_array() {
        Some(items) => puts_media_items(items),
        _ => {
            if token.is_none() {
                println!("no photos were found");
            }
        }
    }

    if let Some(next_token) = response["nextPageToken"].as_str() {
        return list_all_album_contents(client, access_token, album_id, Some(next_token));
    }

    Ok(())
}

pub fn execute_photos(args: &ArgMatches) -> Result<(), Error> {
    let config = Config::load("default")?;
    let client = client().unwrap();

    match args.value_of("album_id") {
        Some(album_id) => list_all_album_contents(&client, &config.access_token, album_id, None),
        _ => list_all_library_contents(&client, &config.access_token, None)
    }
}
