use std::error::Error;
use std::fmt;

pub struct ReadStreamError {
    buf: [u8; 1024],
}

impl ReadStreamError {
    pub fn from(request: [u8; 1024]) -> Self {
        Self { buf: request }
    }
}

impl fmt::Display for ReadStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Something went wrong when reading the request buffer's stream."
        )
    }
}

impl fmt::Debug for ReadStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ReadStreamError")
            .field("request", &self.buf)
            .finish()
    }
}

impl Error for ReadStreamError {}
