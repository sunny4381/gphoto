use hyper;
use hyper::header::{UserAgent, Authorization};

use url::form_urlencoded;

use serde_json;

use time::{self, Timespec};

use super::Args;
use super::access::{client, USER_AGENT, GDataVersion};
use config::Config;
use error::Error;

pub fn execute_photos(args: &Args) -> Result<(), Error> {
    let config = try!(Config::load("default"));

    let user_id = "default";
    let url = match args.flag_album {
        Some(ref album_id) => format!("https://picasaweb.google.com/data/feed/api/user/{}/albumid/{}", user_id, album_id),
        _ => format!("https://picasaweb.google.com/data/feed/api/user/{}", user_id),
    };

    let params: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("kind", "photo")
        .append_pair("alt", "json")
        .append_pair("max-results", &args.flag_max.clone().unwrap_or("10".to_string()))
        .finish();

    let client = client().unwrap();
    let res = try!(client.get(&format!("{}?{}", url, params))
        .header(Authorization(format!("Bearer {}", config.access_token)))
        .header(GDataVersion("3".to_string()))
        .header(UserAgent(USER_AGENT.to_owned()))
        .send());

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HttpError(res.status));
    }

    // std::io::copy(&mut res, &mut std::io::stdout()).unwrap();

    let photos_json: serde_json::Value = try!(serde_json::from_reader(res));

    for entry in photos_json["feed"]["entry"].as_array().unwrap() {
        // println!("{:?}", entry);
        let id = entry["gphoto$id"]["$t"].as_str();
        let title = entry["title"]["$t"].as_str();
        let access = entry["gphoto$access"]["$t"].as_str();
        let timestamp = entry["gphoto$timestamp"]["$t"].as_str();
        let width = entry["gphoto$width"]["$t"].as_str();
        let height = entry["gphoto$height"]["$t"].as_str();
        let size = entry["gphoto$size"]["$t"].as_str();
        let tm = timestamp.map(|v| v.parse::<i64>().unwrap())
            .map(|v| Timespec { sec: v / 1000, nsec: (v % 1000) as i32 })
            .map(|v| time::at_utc(v))
            .map(|v| time::strftime("%FT%TZ", &v).unwrap());
        println!("{}\t{}\t{}\t{}\t{}x{}\t{}", id.unwrap_or(""), title.unwrap_or(""), access.unwrap_or(""), &tm.unwrap_or("".to_string()), width.unwrap_or(""), height.unwrap_or(""), size.unwrap_or(""));
    }

    return Ok(());
}
