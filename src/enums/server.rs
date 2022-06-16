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

#[allow(dead_code)]
pub enum ServerError {
    ParseIntegerError(ParseIntError),
    ParseUtf8Error(Utf8Error),
    StreamError,
    BufferHeaderError,
    MissingHeaderError,
    CorsError,
}

impl From<ParseIntError> for ServerError {
    fn from(error: ParseIntError) -> Self {
        ServerError::ParseIntegerError(error)
    }
}

impl From<Utf8Error> for ServerError {
    fn from(error: Utf8Error) -> Self {
        ServerError::ParseUtf8Error(error)
    }
}
