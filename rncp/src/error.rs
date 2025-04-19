use core::{error, fmt, result, str::Utf8Error};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    InvalidValue,
    InvalidUtf8Arg,
    MissingArg,
    MissingArgValue,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidValue => {
                write!(f, "Invalid argument value")
            }
            Self::InvalidUtf8Arg => {
                write!(f, "Invalid argument format (Non UTF-8")
            }
            Self::MissingArg => {
                write!(f, "Missing argument")
            }
            Self::MissingArgValue => {
                write!(f, "Missing argument value")
            }
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Self {
        Error::InvalidUtf8Arg
    }
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;
