use std::io::{self,BufRead,Write};

use super::Args;
use super::access::{auth_url, auth_token};
use error::Error;
use config::Config;

// fn flush_stdout() -> Result<(), Error> {
//     try!(io::stdout().flush());
//     return Ok(());
// }
fn prompt(label: &str) -> Result<(), Error> {
    print!("put your {}: ", label);
    try!(io::stdout().flush());
    return Ok(());
}

fn read_from_stdin(label: &str) -> Result<String, Error> {
    loop {
        try!(prompt(label));

        let stdin = io::stdin();
        let input = stdin.lock().lines().next();
        if input.is_none() {
            continue;
        }
        let line = try!(input.unwrap());
        if line.len() > 0 {
            return Ok(line);
        }
    }
}

fn read_code(client_id: &str) -> Result<String, Error> {
    println!("visit {}", auth_url(client_id));

    loop {
        try!(prompt("code"));

        let stdin = io::stdin();
        let input = stdin.lock().lines().next();
        if input.is_none() {
            continue;
        }
        let line = try!(input.unwrap());
        if line.len() > 0 {
            return Ok(line);
        }
    }
}

pub fn execute_init(args: &Args) -> Result<(), Error> {
    let client_id = try!(match args.arg_clinet_id {
        Some(ref clinet_id) => Ok(clinet_id.clone()),
        _ => read_from_stdin("client id"),
    });
    let client_secret = try!(match args.flag_secret {
        Some(ref client_secret) => Ok(client_secret.clone()),
        _ => read_from_stdin("client secret"),
    });

    let code = try!(read_code(&client_id));
    let token = try!(auth_token(&client_id, &client_secret, &code));

    let config = Config {
        client_id: client_id,
        client_secret: client_secret,
        access_token: token.access_token,
        expires_in: token.expires_in,
        refresh_token: token.refresh_token.unwrap()
    };
    try!(config.save("default"));

    return Ok(());
}
