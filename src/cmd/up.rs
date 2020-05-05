use std::fs::File;
use std::io::Read;
use std::path::Path;

use hyper::header::{UserAgent, Authorization, ContentType};

use mime;

use super::Args;
use goauth::{client, USER_AGENT, XGoogUploadContentType, XGoogUploadProtocol};
use config::Config;
use error::Error;

const UPLOAD_API_URL: &'static str = "https://photoslibrary.googleapis.com/v1/uploads";
const MEDIA_ITEM_CREATE_API_URL: &'static str = "https://photoslibrary.googleapis.com/v1/mediaItems:batchCreate";

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

fn upload_image(client: &hyper::Client, access_token: &str, filepath: &Path) -> Result<String, Error> {
    let mime_type = image_mime_type(&filepath)
        .unwrap_or("application/octet-stream".parse().unwrap());

    let mut file: File = File::open(&filepath)?;

    let request = client.post(UPLOAD_API_URL)
        .header(Authorization(format!("Bearer {}", access_token)))
        .header(UserAgent(USER_AGENT.to_owned()))
        .header(ContentType("application/octet-stream".parse().unwrap()))
        .header(XGoogUploadContentType(mime_type))
        .header(XGoogUploadProtocol("raw".to_string()))
        .body(&mut file);

    let mut res = request.send()?;
    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HttpError(res.status));
    }

    let mut token = String::new();
    let size = res.read_to_string(&mut token)?;
    if size == 0 {
        return Ok("".to_string());
    }

    Ok(token)
}

fn create_media_item(client: &hyper::Client, access_token: &str, filepath: &Path, name: &Option<&str>, album_id: &Option<&str>, upload_token: &str) -> Result<(), Error> {
    let mut request_body = json!({
        "newMediaItems": [
            {
                "description": name.unwrap_or_else(|| filepath.file_name().unwrap().to_str().unwrap()),
                "simpleMediaItem": {
                    "fileName": filepath.file_name().unwrap().to_str().unwrap_or("filename.png"),
                    "uploadToken": upload_token
                }
            }
        ]
    });

    if let Some(id) = album_id {
        request_body.as_object_mut().unwrap().insert(String::from("albumId"), json!(id));
    }

    let request_json = request_body.to_string();

    let req = client.post(MEDIA_ITEM_CREATE_API_URL)
        .header(Authorization(format!("Bearer {}", access_token)))
        .header(UserAgent(USER_AGENT.to_owned()))
        .header(ContentType("application/json".parse().unwrap()))
        .body(request_json.as_str());

    let res = req.send()?;
    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HttpError(res.status));
    }

    Ok(())
}

pub fn execute_up(args: &Args) -> Result<(), Error> {
    let config = Config::load("default")?;

    let filepath = match args.arg_file {
        Some(ref file) => Path::new(file),
        _ => panic!("specify file"),
    };
    let name: Option<&str> = args.flag_name.as_ref().map(|s| s.as_str());
    let album_id: Option<&str> = args.flag_album.as_ref().map(|s| s.as_str());

    let client = client()?;

    let upload_token = upload_image(&client, &config.access_token, &filepath)?;
    create_media_item(&client, &config.access_token, filepath, &name, &album_id, upload_token.as_str())
}
