use super::Args;
use goauth::{user_info};
use config::Config;
use error::Error;

pub fn execute_whoami(_: &Args) -> Result<(), Error> {
    let config = try!(Config::load("default"));
    let info = try!(user_info(&config.access_token));

    println!("{}", info.email);

    Ok(())
}
