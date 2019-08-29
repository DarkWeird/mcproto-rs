use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result::Result as StdResult;

use serde;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Serde(String),
    NBTError(nbt::Error),
    UnsupportedType(&'static str),
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::IoError(ref e) => e.fmt(f),
            &Error::Serde(ref msg) => write!(f, "{}", msg),
             Error::UnsupportedType(ref t) => write!(f,"unsupported  type {}", t),
            // Static messages should suffice for the remaining errors.
            other => write!(f, "{}", other.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e) => e.description(),
            Error::Serde(ref msg) => &msg[..],
            Error::NBTError(ref e) => e.description(),
            Error::UnsupportedType(_) => "unsupported  type",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::IoError(ref e) => e.source(),
            Error::NBTError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Serde(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Serde(msg.to_string())
    }
}
