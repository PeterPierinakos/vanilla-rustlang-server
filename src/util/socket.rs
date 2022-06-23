use std::io::Read;
use std::net::TcpStream;
use std::str;

use super::status::StatusCode;
use super::{
    headers::{find_buf_headers, Header},
    response::ErrorResponse,
};

/* Verifies that the socket has valid request data, otherwise error. */
pub fn read_stream(stream: &mut TcpStream) -> Result<(Header, [u8; 1024]), StatusCode> {
    let mut buf = [0; 1024];

    match stream.read(&mut buf) {
        Ok(_val) => Ok((find_buf_headers(&buf)?, buf)),
        Err(_) => Err(400),
    }
}

pub fn parse_utf8(headers: &Header, buf: &[u8; 1024]) -> Result<String, ErrorResponse> {
    let parsed_utf8 = str::from_utf8(buf);

    match parsed_utf8 {
        Ok(string) => Ok(string.to_string()),
        Err(_) => Err((headers.clone(), 400)),
    }
}
