//!!!!!!!! TODO ZOV ZVO SAFETY: TODO
// хз как тебе пометить чтобы ты заметил,
// короче оставляй тут только парсерные ошибки, я же перенес
// env, так что ну, думай теперь сам чо надо а чо нет

use core::{error, fmt, result, str::Utf8Error};

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Error<'a> {
    InvalidValue,
    InvalidUtf8Arg,
    MissingArg,
    MissingArgValue,
    DuplicatedArg,
    UnknownLongOption(&'a str),
    UnknownShortOption(&'a str),
    ParseError(ParserError),
}

/// 'static, cuz arg has 'static lifetime
#[derive(Debug, PartialEq)]
pub enum ParserError {
    IntParse(&'static str),
    StrParse(&'static str),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue => write!(f, "Invalid argument value"),
            Self::InvalidUtf8Arg => write!(f, "Invalid argument format (Non UTF-8)"),
            Self::MissingArg => write!(f, "Missing argument"),
            Self::MissingArgValue => write!(f, "Missing option value"),
            Self::DuplicatedArg => write!(f, "Duplicated argument"),
            Self::ParseError(e) => write!(f, "{e}"),
            Self::UnknownLongOption(opt) => write!(f, "Unknown long option --{opt}"),
            Self::UnknownShortOption(opt) => write!(f, "Unknown short option -{opt}")
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntParse(arg) => write!(f, "Error while parsing {arg} as integer"),
            Self::StrParse(arg) => write!(f, "Error while parsing {arg} as string"),
        }
    }
}

impl From<ParserError> for Error<'_> {
    fn from(e: ParserError) -> Self {
        Error::ParseError(e)
    }
}

impl From<Utf8Error> for Error<'_> {
    fn from(_: Utf8Error) -> Self {
        Error::InvalidUtf8Arg
    }
}

impl error::Error for Error<'_> {}

pub type Result<'a, T> = result::Result<T, Error<'a>>;
