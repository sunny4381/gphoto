use std::env;
use std::fs;
use std::io::Write;
use std::path;

use serde_json;

use error::Error;

#[derive(Debug)]
pub struct Config {
    pub access_token: String,
    pub client_id: String,
    pub client_secret: String,
    pub expires_in: u64,
    pub refresh_token: String,
}

fn home_dir() -> Result<path::PathBuf, env::VarError> {
    match env::var("GPHOTO_HOME") {
        Ok(path) => return Ok(path::PathBuf::from(path)),
        Err(_) => (),
    };

    let home = try!(env::var("HOME"));
    let mut homepath = path::PathBuf::from(home);
    homepath.push(".gphoto");
    return Ok(homepath);
}

impl Config {
    pub fn load(profile: &str) -> Result<Config, Error> {
        let gphoto_dir = try!(home_dir());
        let filepath = gphoto_dir.as_path().join(profile);
        let file = try!(fs::File::open(filepath));

        let token_json: serde_json::Value = try!(serde_json::from_reader(file));
        return Ok(Config {
            access_token: String::from(token_json["access_token"].as_str().unwrap()),
            client_id: String::from(token_json["client_id"].as_str().unwrap()),
            client_secret: String::from(token_json["client_secret"].as_str().unwrap()),
            expires_in: token_json["expires_in"].as_u64().unwrap(),
            refresh_token: String::from(token_json["refresh_token"].as_str().unwrap()),
        });
    }

    pub fn save(&self, profile: &str) -> Result<(), Error> {
        let cfg = json!({
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "access_token": self.access_token,
            "expires_in": self.expires_in,
            "refresh_token": self.refresh_token,
        });

        let gphoto_dir = try!(home_dir());
        try!(fs::create_dir_all(gphoto_dir.as_path()));

        let filepath = gphoto_dir.as_path().join(profile);
        let mut file = try!(fs::File::create(filepath));

        file.write_all(cfg.to_string().as_bytes()).unwrap();

        return Ok(());
    }
}
