use std::fs::File;
use std::io::Read;
use std::path::Path;

use clap::ArgMatches;
use mime;
use reqwest;
use reqwest::blocking::Client;
use serde_json::json;

use goauth::USER_AGENT;
use config::Config;
use error::Error;

const UPLOAD_API_URL: &'static str = "https://photoslibrary.googleapis.com/v1/uploads";
const MEDIA_ITEM_CREATE_API_URL: &'static str = "https://photoslibrary.googleapis.com/v1/mediaItems:batchCreate";

fn image_mime_type(filename: &Path) -> Option<mime::Mime> {
    match filename.extension() {
        Some(os_str) => {
            if os_str == "gif" {
                Some(mime::IMAGE_GIF)
            } else if os_str == "png" {
                Some(mime::IMAGE_PNG)
            } else if os_str == "jpg" || os_str == "jpeg" {
                Some(mime::IMAGE_JPEG)
            } else {
                None
            }
        },
        _ => None
    }
}

fn upload_image(client: &Client, access_token: &str, filepath: &Path) -> Result<String, Error> {
    let mime_type: mime::Mime = image_mime_type(&filepath)
        .unwrap_or(mime::APPLICATION_OCTET_STREAM);

    let file: File = File::open(&filepath)?;

    let req = client.post(UPLOAD_API_URL)
        .bearer_auth(access_token)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, "application/octet-stream")
        .header("X-Goog-Upload-Content-Type", mime_type.as_ref())
        .header("X-Goog-Upload-Protocol", "raw")
        .body(file);

    let mut res = req.send()?;
    if !res.status().is_success() {
        return Err(Error::from(res));
    }

    let mut upload_token = String::new();
    let size = res.read_to_string(&mut upload_token)?;
    if size == 0 {
        panic!("unable to get upload token");
    }

    Ok(upload_token)
}

fn create_media_item(client: &Client, access_token: &str, filepath: &Path, description: &Option<&str>, album_id: &Option<&str>, filename: &Option<&str>, upload_token: &str) -> Result<(), Error> {
    let mut request_body = json!({
        "newMediaItems": [
            {
                "description": description.unwrap_or_else(|| filepath.file_name().unwrap().to_str().unwrap()),
                "simpleMediaItem": {
                    "fileName": filename.unwrap_or_else(|| filepath.file_name().unwrap().to_str().unwrap_or("filename.png")),
                    "uploadToken": upload_token
                }
            }
        ]
    });

    if let Some(id) = album_id {
        request_body.as_object_mut().unwrap().insert(String::from("albumId"), json!(id));
    }

    let request_json: String = request_body.to_string();

    let req = client.post(MEDIA_ITEM_CREATE_API_URL)
        .bearer_auth(access_token)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(request_json);

    let res = req.send()?;
    if !res.status().is_success() {
        return Err(Error::from(res));
    }

    Ok(())
}

pub fn execute_up(args: &ArgMatches) -> Result<(), Error> {
    let config = Config::load("default")?;

    let filepath = match args.value_of("file") {
        Some(file) => Path::new(file),
        _ => panic!("specify file"),
    };
    let description: Option<&str> = args.value_of("description");
    let album_id: Option<&str> = args.value_of("album_id");
    let filename: Option<&str> = args.value_of("filename");

    let client = Client::new();

    let upload_token = upload_image(&client, &config.access_token, &filepath)?;
    create_media_item(&client, &config.access_token, filepath, &description, &album_id, &filename, upload_token.as_str())
}
