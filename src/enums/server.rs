use crate::structs::readstreamerror::ReadStreamError;
use std::num::ParseIntError;
use std::str::Utf8Error;

#[allow(dead_code)]
pub enum StatusCode {
    OK,
    BadRequest,
    NotFound,
    InternalServerError,
    MethodNotAllowed,
}

pub enum ServerError {
    ParseIntegerError(ParseIntError),
    ParseUtf8Error(Utf8Error),
    StreamError(ReadStreamError),
}

impl From<ParseIntError> for ServerError {
    fn from(error: ParseIntError) -> Self {
        ServerError::ParseIntegerError(error)
    }
}

impl From<ReadStreamError> for ServerError {
    fn from(error: ReadStreamError) -> Self {
        ServerError::StreamError(error)
    }
}

impl From<Utf8Error> for ServerError {
    fn from(error: Utf8Error) -> Self {
        ServerError::ParseUtf8Error(error)
    }
}
