use std::{fmt, num::ParseIntError, str::Utf8Error};

#[derive(Debug)]
pub enum ServerError {
    ParseUtf8Error(Utf8Error),
    IOError(std::io::Error),
    ParseIntError(ParseIntError),
    UnknownError,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ServerError::UnknownError)
    }
}

impl std::error::Error for ServerError {}

impl From<Utf8Error> for ServerError {
    fn from(e: Utf8Error) -> Self {
        Self::ParseUtf8Error(e)
    }
}

impl From<ParseIntError> for ServerError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
