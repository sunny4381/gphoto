extern crate docopt;
extern crate env_logger;
extern crate rustc_serialize;
#[macro_use]
extern crate hyper;
extern crate hyper_native_tls;
extern crate native_tls;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate time;
extern crate mime;

mod cmd;
mod config;
mod error;
mod goauth;

use std::io::{self, Write};
use docopt::Docopt;

use cmd::execute;
use error::Error;

const USAGE: &'static str = r#"
Google Photo Uploader.

Usage:
  gphoto init <clinet-id> [--secret=<secret>]
  gphoto refresh
  gphoto whoami
  gphoto albums [--user-id=<user-id>]
  gphoto photos [--album=<album>] [--max=<max>] [--user-id=<user-id>]
  gphoto up <file> [--name=<name>] [--album=<album>] [--user-id=<user-id>]
  gphoto (-h | --help)
Options:
  -h, --help     Show this screen.
  --secret=<secret> Specify client secret.
  --album=<album>  Sepcify name of album.
  --max=<max>  Sepcify max results of photos [default: 10].
  --name=<name>  Sepcify name of photo.
  --user-id=<user-id> Specify user id of Google [default: "default"].
"#;

#[derive(Debug, RustcDecodable)]
pub struct Args {
    flag_secret: Option<String>,
    flag_album: Option<String>,
    flag_max: Option<String>,
    flag_name: Option<String>,
    flag_user_id: Option<String>,
    arg_clinet_id: Option<String>,
    arg_file: Option<String>,
    cmd_init: bool,
    cmd_refresh: bool,
    cmd_whoami: bool,
    cmd_albums: bool,
    cmd_photos: bool,
    cmd_up: bool,
}

fn main() {
    env_logger::init().unwrap();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    match execute(&args) {
        Ok(_) => (),
        Err(ref e) => abort(e),
    };
}

pub fn abort(e: &Error) {
    writeln!(&mut io::stderr(), "{}", e).unwrap();
    ::std::process::exit(1)
}
