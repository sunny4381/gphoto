use hyper::header::{UserAgent, Authorization};

use hyper;
use serde_json;

use super::Args;
use super::access::{client, USER_AGENT, GDataVersion};
use config::Config;
use error::Error;

pub fn execute_albums(_: &Args) -> Result<(), Error> {
    let config = try!(Config::load("default"));
    let access_token = config.access_token;

    let user_id = "default";
    let url = format!("https://picasaweb.google.com/data/feed/api/user/{}?alt=json", user_id);
    let client = try!(client());
    let res = try!(client.get(&url)
        .header(Authorization(format!("Bearer {}", access_token)))
        .header(GDataVersion("3".to_string()))
        .header(UserAgent(USER_AGENT.to_owned()))
        .send());

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HttpError(res.status));
    }

    let albums_json: serde_json::Value = try!(serde_json::from_reader(res));

    for entry in albums_json["feed"]["entry"].as_array().unwrap() {
        let id = entry["gphoto$id"]["$t"].as_str();
        let title = entry["title"]["$t"].as_str();
        match (id, title) {
            (Some(id), Some(title)) => println!("{}\t{}", id, title),
            _ => ()
        };
    };

    Ok(())
}
