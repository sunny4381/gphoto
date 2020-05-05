use clap::ArgMatches;

use goauth::{user_info};
use config::Config;
use error::Error;

pub fn execute_whoami(_args: &ArgMatches) -> Result<(), Error> {
    let config = Config::load("default")?;
    let info = user_info(&config.access_token)?;

    println!("{}", info.email);

    Ok(())
}
