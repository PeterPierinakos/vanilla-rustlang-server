#![deny(unsafe_code)]

use std::sync::Arc;

use vrs::configuration::MULTITHREADING;
use vrs::structs::server::Server;

pub fn main() -> std::io::Result<()> {
    let server = Arc::new(Server::new()?);
    match MULTITHREADING {
        true => server.start_multithread()?,
        false => server.start_singlethread()?,
    }
    Ok(())
}
