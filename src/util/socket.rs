use std::io::Read;
use std::net::TcpStream;
use std::str;

/* Verifies that the socket has valid request data, otherwise error. */
pub fn handle_stream(stream: &mut TcpStream) -> Option<[u8; 1024]> {
    let mut buf = [0; 1024];

    match stream.read(&mut buf) {
        Ok(_val) => Some(buf),
        Err(_err) => None,
    }
}

pub fn parse_utf8(buf: &[u8; 1024]) -> Option<String> {
    let parsed_utf8 = str::from_utf8(buf);    

    match parsed_utf8 {
        Ok(string) => Some(string.to_string()),
        Err(_err) => None,
    }
}
