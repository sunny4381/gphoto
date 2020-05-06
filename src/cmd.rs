mod albums;
mod album_create;
mod init;
mod photos;
mod refresh;
mod up;
mod whoami;

use clap::ArgMatches;

use self::init::execute_init;
use self::refresh::execute_refresh;
use self::up::execute_up;
use self::albums::execute_albums;
use self::album_create::execute_album_create;
use self::photos::execute_photos;
use self::whoami::execute_whoami;
use crate::error::Error;

pub fn execute(args: &ArgMatches) -> Result<(), Error> {
    if let Some(args) = args.subcommand_matches("init") {
        return execute_init(args);
    } else if let Some(args) = args.subcommand_matches("refresh") {
        return execute_refresh(args);
    } else if let Some(args) = args.subcommand_matches("whoami") {
        return execute_whoami(args);
    } else if let Some(args) = args.subcommand_matches("albums") {
        if let Some(args) = args.subcommand_matches("list") {
            return execute_albums(args);
        } else if let Some(args) = args.subcommand_matches("create") {
            return execute_album_create(args);
        }
    } else if let Some(args) = args.subcommand_matches("photos") {
        if let Some(args) = args.subcommand_matches("list") {
            return execute_photos(args);
        } else if let Some(args) = args.subcommand_matches("up") {
            return execute_up(args);
        }
    }

    return Err(Error::UnknownCommandError);
}
