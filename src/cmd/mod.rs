mod albums;
mod album_create;
mod init;
mod photos;
mod refresh;
mod up;
mod whoami;

use super::Args;
use self::init::execute_init;
use self::refresh::execute_refresh;
use self::up::execute_up;
use self::albums::execute_albums;
use self::album_create::execute_album_create;
use self::photos::execute_photos;
use self::whoami::execute_whoami;
use error::Error;

pub fn execute(args: &Args) -> Result<(), Error> {
    if args.cmd_init {
        return execute_init(args);
    } else if args.cmd_refresh {
        return execute_refresh(args);
    } else if args.cmd_whoami {
        return execute_whoami(args);
    } else if args.cmd_albums {
        return execute_albums(args);
    } else if args.cmd_album_create {
        return execute_album_create(args);
    } else if args.cmd_photos {
        return execute_photos(args);
    } else if args.cmd_up {
        return execute_up(args);
    }

    return Err(Error::UnknownCommandError);
}
