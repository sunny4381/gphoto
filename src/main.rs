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

const USAGE: &'static str = r#"
Google Photo Uploader.

Usage:
  gphoto init <clinet-id> [--secret=<secret>]
  gphoto refresh
  gphoto albums
  gphoto photos [--album=<album>] [--max=<max>]
  gphoto up <file> [--name=<name>] [--album=<album>]
  gphoto (-h | --help)
Options:
  -h, --help     Show this screen.
  --secret=<secret> Specify client secret.
  --album=<album>  Sepcify name of album.
  --max=<max>  Sepcify max results of photos [default: 10].
  --name=<name>  Sepcify name of photo.
"#;

#[derive(Debug, RustcDecodable)]
pub struct Args {
    flag_secret: Option<String>,
    flag_album: Option<String>,
    flag_max: Option<String>,
    flag_name: Option<String>,
    arg_clinet_id: Option<String>,
    arg_file: Option<String>,
    cmd_init: bool,
    cmd_refresh: bool,
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
        Err(ref e) => abort(format!("{}", e).as_str()),
    };
}

pub fn abort(why: &str) {
    write!(&mut io::stderr(), "{}", why).unwrap();
    ::std::process::exit(1)
}
