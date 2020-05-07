pub(crate) mod init;
pub(crate) mod refresh;
pub(crate) mod whoami;
pub(crate) mod albums;
pub(crate) mod photos;

use clap::ArgMatches;

use self::init::execute_init;
use self::refresh::execute_refresh;
use self::whoami::execute_whoami;
use self::albums::list::execute_albums_list;
use self::albums::create::execute_albums_create;
use self::photos::list::execute_photos_list;
use self::photos::up::execute_photos_up;
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
            return execute_albums_list(args);
        } else if let Some(args) = args.subcommand_matches("create") {
            return execute_albums_create(args);
        }
    } else if let Some(args) = args.subcommand_matches("photos") {
        if let Some(args) = args.subcommand_matches("list") {
            return execute_photos_list(args);
        } else if let Some(args) = args.subcommand_matches("up") {
            return execute_photos_up(args);
        }
    }

    return Err(Error::UnknownCommandError);
}
