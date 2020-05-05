use super::Args;
use goauth;
use config::Config;
use error::Error;

pub fn execute_refresh(_: &Args) -> Result<(), Error> {
    let config = Config::load("default")?;
    let token = goauth::refresh_token(&config.client_id, &config.client_secret, &config.refresh_token)?;

    let new_config = Config {
        client_id: config.client_id,
        client_secret: config.client_secret,
        access_token: token.access_token,
        expires_in: token.expires_in,
        refresh_token: config.refresh_token,
    };
    new_config.save("default")?;

    return Ok(());
}
