use crate::structs::readstreamerror::ReadStreamError;
use std::io::Read;
use std::net::TcpStream;
use std::str;
use std::str::Utf8Error;

/* Verifies that the socket has valid request data, otherwise error. */
pub fn read_stream(stream: &mut TcpStream) -> Result<[u8; 1024], ReadStreamError> {
    let mut buf = [0; 1024];

    match stream.read(&mut buf) {
        Ok(_val) => Ok(buf),
        Err(_) => Err(ReadStreamError::from(buf)),
    }
}

pub fn parse_utf8(buf: &[u8; 1024]) -> Result<String, Utf8Error> {
    let parsed_utf8 = str::from_utf8(buf);

    match parsed_utf8 {
        Ok(string) => Ok(string.to_string()),
        Err(err) => Err(err),
    }
}
