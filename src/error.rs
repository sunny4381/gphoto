use std::env;
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
    ConfigError(String),
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
            Error::ConfigError(ref msg) => write!(f, "Config error: {}", msg),
            Error::UnknownCommandError => write!(f, "Unknown Command"),
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
