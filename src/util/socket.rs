use crate::enums::server::ServerError;
use std::net::TcpStream;
use std::str;
use std::{cell::RefCell, io::Read};

use super::{
    headers::{find_buf_headers, Header},
    response::ErrorResponse,
};

/* Verifies that the socket has valid request data, otherwise error. */
pub fn read_stream<'a>(
    stream: &'a mut TcpStream,
) -> Result<(Header, [u8; 1024]), ErrorResponse<'a>> {
    let mut buf = [0; 1024];

    match stream.read(&mut buf) {
        Ok(_val) => Ok((find_buf_headers(&buf)?, buf)),
        Err(_) => Err((
            RefCell::from(find_buf_headers(&buf)?),
            ServerError::StreamError,
        )),
    }
}

pub fn parse_utf8<'a>(
    headers: &'a RefCell<Header>,
    buf: &'a [u8; 1024],
) -> Result<String, ErrorResponse<'a>> {
    let parsed_utf8 = str::from_utf8(buf);

    match parsed_utf8 {
        Ok(string) => Ok(string.to_string()),
        Err(err) => Err((headers.clone(), ServerError::ParseUtf8Error(err))),
    }
}
