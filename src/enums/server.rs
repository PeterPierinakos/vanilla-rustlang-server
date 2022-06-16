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

impl<'a> StatusCode {
    pub fn as_u16(&self) -> u16 {
        match self {
            StatusCode::OK => 200,
            StatusCode::BadRequest => 400,
            StatusCode::NotFound => 404,
            StatusCode::InternalServerError => 500,
            StatusCode::MethodNotAllowed => 405,
        }
    }

    pub fn as_str(&self) -> &'a str {
        match self {
            StatusCode::OK => "OK",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
        }
    }
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
