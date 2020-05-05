use hyper::header::{UserAgent, Authorization, ContentType};

use super::Args;
use goauth::{client, USER_AGENT};
use config::Config;
use error::Error;

const ALBUM_API_URL: &'static str = "https://photoslibrary.googleapis.com/v1/albums";

pub fn execute_album_create(args: &Args) -> Result<(), Error> {
    let name = match args.flag_name {
        Some(ref name) => name,
        _ => panic!("specify album name"),
    };

    let config = Config::load("default")?;
    let access_token = config.access_token;

    let client = client()?;

    let request_body = json!({
        "album": {
            "title": name,
        }
    });
    let request_json = request_body.to_string();
    let req = client.post(ALBUM_API_URL)
        .header(Authorization(format!("Bearer {}", access_token)))
        .header(UserAgent(USER_AGENT.to_owned()))
        .header(ContentType("application/json".parse().unwrap()))
        .body(request_json.as_str());

    let res = req.send()?;
    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HttpError(res.status));
    }

    let album_json: serde_json::Value = serde_json::from_reader(res)?;
    let id = album_json["id"].as_str().unwrap();
    let product_url = album_json["productUrl"].as_str().unwrap();
    // album_json["title"].as_str();
    // album_json["isWriteable"].as_str();

    println!("created {}({})", id, product_url);

    Ok(())
}
