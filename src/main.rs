#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate mime;
extern crate reqwest;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate url;

mod cmd;
mod config;
mod error;
mod goauth;

use std::io::{self, Write};

use cmd::execute;
use error::Error;

fn main() {
    let args = clap_app!(myapp =>
        (author: "NAKANO Hideo. <pinarello.marvel@gmail.com>")
        (about: "Google Photo Uploader")
        (@subcommand init =>
            (about: "initialize environment")
            (@arg client_id: +required "client id")
            (@arg client_secret: +required "client secret")
        )
        (@subcommand refresh =>
            (about: "refresh access token")
        )
        (@subcommand whoami =>
            (about: "print who am I")
        )
        (@subcommand albums =>
            (@subcommand list =>
                (about: "show all albums")
            )
            (@subcommand create =>
                (about: "create new album")
                (@arg name: +required "album name")
            )
        )
        (@subcommand photos =>
            (@subcommand list =>
                (about: "show all photos")
                (@arg album_id: --album_id "album id to show")
            )
            (@subcommand up =>
                (about: "upload photo")
                (@arg file: +required "file to upload")
                (@arg description: --description "photo description")
                (@arg album_id: --album_id "album id to put")
                (@arg filename: --filename "filename of photo")
            )
        )
    ).get_matches();

    env_logger::init();

    match execute(&args) {
        Ok(_) => (),
        Err(ref e) => abort(e),
    };
}

pub fn abort(e: &Error) {
    writeln!(&mut io::stderr(), "{}", e).unwrap();
    ::std::process::exit(1)
}
