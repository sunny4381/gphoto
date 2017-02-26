use super::Args;
use super::access;
use config::Config;
use error::Error;

pub fn execute_refresh(_: &Args) -> Result<(), Error> {
    let config = try!(Config::load("default"));
    let token = try!(access::refresh_token(&config.client_id, &config.client_secret, &config.refresh_token));

    let new_config = Config {
        client_id: config.client_id,
        client_secret: config.client_secret,
        access_token: token.access_token,
        expires_in: token.expires_in,
        refresh_token: config.refresh_token,
    };
    try!(new_config.save("default"));

    return Ok(());
}
