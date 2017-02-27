use std::fs::File;
use std::path::Path;

use hyper::header::{UserAgent, Authorization, ContentType, ContentLength};

use mime;

use super::Args;
use goauth::{client, USER_AGENT, GDataVersion, Slug};
use config::Config;
use error::Error;

fn image_mime_type(filename: &Path) -> Option<mime::Mime> {
    match filename.extension() {
        Some(os_str) => {
            if os_str == "gif" {
                Some("image/gif".parse().unwrap())
            } else if os_str == "png" {
                Some("image/png".parse().unwrap())
            } else if os_str == "jpg" || os_str == "jpeg" {
                Some("image/jpeg".parse().unwrap())
            } else {
                None
            }
        },
        _ => None
    }
}

pub fn execute_up(args: &Args) -> Result<(), Error> {
    let config = try!(Config::load("default"));

    let user_id = "default";
    let url = match args.flag_album {
        Some(ref album_id) => format!("https://picasaweb.google.com/data/feed/api/user/{}/albumid/{}", user_id, album_id),
        _ => format!("https://picasaweb.google.com/data/feed/api/user/{}", user_id),
    };

    let filepath = match args.arg_file {
        Some(ref file) => Path::new(file),
        _ => panic!("specify up file"),
    };

    let mut file: File = try!(File::open(&filepath));
    let size = file.metadata().unwrap().len();

    let mime_type = image_mime_type(&filepath)
        .unwrap_or("application/octet-stream".parse().unwrap());

    let client = try!(client());
    let mut req = client.post(&url)
        .body(&mut file)
        .header(Authorization(format!("Bearer {}", config.access_token)))
        .header(GDataVersion("3".to_string()))
        .header(ContentType(mime_type))
        .header(ContentLength(size))
        .header(UserAgent(USER_AGENT.to_owned()));

    req = match args.flag_name {
        Some(ref name) => req.header(Slug(name.clone())),
        _ => req.header(Slug(filepath.file_name().unwrap().to_os_string().into_string().unwrap()))
    };

    let res = try!(req.send());

    println!("{}", res.status);

    return Ok(());
}
