use std::env;
use std::error;
use std::io;
use std::fmt;

use hyper;

use serde_json;

use native_tls;

#[derive(Debug)]
pub enum Error {
    EnvError(env::VarError),
    IoError(io::Error),
    HyperError(hyper::error::Error),
    HttpError(hyper::status::StatusCode),
    SerdeError(serde_json::error::Error),
    NativeTlsError(native_tls::Error),
    UnknownCommandError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::EnvError(ref err) => write!(f, "IO error: {}", err),
            Error::IoError(ref err) => write!(f, "IO error: {}", err),
            Error::HyperError(ref err) => write!(f, "Hyper error: {}", err),
            Error::HttpError(ref status) => write!(f, "HTTP error: {}", status),
            Error::SerdeError(ref err) => write!(f, "Serde error: {}", err),
            Error::NativeTlsError(ref err) => write!(f, "NativeTls error: {}", err),
            Error::UnknownCommandError => write!(f, "Unknown Command"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        // 下層のエラーは両方ともすでに `Error` を実装しているので、
        // それらの実装に従います。
        match *self {
            Error::EnvError(ref err) => err.description(),
            Error::IoError(ref err) => err.description(),
            Error::HyperError(ref err) => err.description(),
            Error::HttpError(ref status) => status.canonical_reason().unwrap(),
            Error::SerdeError(ref err) => err.description(),
            Error::NativeTlsError(ref err) => err.description(),
            Error::UnknownCommandError => "unknown command",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // 注意：これらは両方とも `err` を、その具象型（`&io::Error` か
            // `&num::ParseIntError` のいずれか）から、トレイトオブジェクト
            // `&Error` へ暗黙的にキャストします。どちらのエラー型も `Error` を
            // 実装しているので、問題なく動きます。
            Error::EnvError(ref err) => Some(err),
            Error::IoError(ref err) => Some(err),
            Error::HyperError(ref err) => Some(err),
            Error::HttpError(_) => None,
            Error::SerdeError(ref err) => Some(err),
            Error::NativeTlsError(ref err) => Some(err),
            Error::UnknownCommandError => None,
        }
    }
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error::EnvError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error::HyperError(err)
    }
}

impl From<hyper::status::StatusCode> for Error {
    fn from(status: hyper::status::StatusCode) -> Error {
        Error::HttpError(status)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::SerdeError(err)
    }
}

impl From<native_tls::Error> for Error {
    fn from(err: native_tls::Error) -> Error {
        Error::NativeTlsError(err)
    }
}
