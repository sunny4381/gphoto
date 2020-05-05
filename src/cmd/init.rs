use std::io::{self,BufRead,Write};

use super::Args;
use goauth::{auth_url, auth_token};
use error::Error;
use config::Config;

fn prompt(label: &str) -> Result<(), Error> {
    print!("put your {}: ", label);
    io::stdout().flush()?;
    return Ok(());
}

fn read_from_stdin(label: &str) -> Result<String, Error> {
    loop {
        prompt(label)?;

        let stdin = io::stdin();
        let input = stdin.lock().lines().next();
        if input.is_none() {
            continue;
        }
        let line = input.unwrap()?;
        if line.len() > 0 {
            return Ok(line);
        }
    }
}

fn read_code(client_id: &str) -> Result<String, Error> {
    println!("visit {}", auth_url(client_id));

    loop {
        prompt("code")?;

        let stdin = io::stdin();
        let input = stdin.lock().lines().next();
        if input.is_none() {
            continue;
        }
        let line = input.unwrap()?;
        if line.len() > 0 {
            return Ok(line);
        }
    }
}

pub fn execute_init(args: &Args) -> Result<(), Error> {
    let client_id = match args.arg_clinet_id {
        Some(ref clinet_id) => Ok(clinet_id.clone()),
        _ => read_from_stdin("client id"),
    }?;
    let client_secret = match args.flag_secret {
        Some(ref client_secret) => Ok(client_secret.clone()),
        _ => read_from_stdin("client secret"),
    }?;

    let code = read_code(&client_id)?;
    let token = auth_token(&client_id, &client_secret, &code)?;

    let config = Config {
        client_id: client_id,
        client_secret: client_secret,
        access_token: token.access_token,
        expires_in: token.expires_in,
        refresh_token: token.refresh_token.unwrap()
    };
    config.save("default")?;

    return Ok(());
}
