mod access;
mod albums;
mod init;
mod photos;
mod refresh;
mod up;

use super::Args;
use self::init::execute_init;
use self::refresh::execute_refresh;
use self::up::execute_up;
use self::albums::execute_albums;
use self::photos::execute_photos;
use error::Error;

pub fn execute(args: &Args) -> Result<(), Error> {
    if args.cmd_init {
        return execute_init(args);
    } else if args.cmd_refresh {
        return execute_refresh(args);
    } else if args.cmd_albums {
        return execute_albums(args);
    } else if args.cmd_photos {
        return execute_photos(args);
    } else if args.cmd_up {
        return execute_up(args);
    }

    return Err(Error::UnknownCommandError);
}
