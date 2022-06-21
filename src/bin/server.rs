#![deny(unsafe_code)]

use std::sync::Arc;

use vrs::configuration::MULTITHREADING;
use vrs::structs::server::Server;

pub fn main() -> std::io::Result<()> {
    let server = Arc::new(Server::new()?);
    if MULTITHREADING {
        server.start_multithread()
    } else {
        server.start_singlethread()
    }
}
