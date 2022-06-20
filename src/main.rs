mod configuration;
mod enums;
mod structs;
mod util;

use std::sync::Arc;

use configuration::MULTITHREADING;
use structs::server::Server;

pub fn main() -> std::io::Result<()> {
    let server = Arc::new(Server::new()?);
    match MULTITHREADING {
        true => server.start_multithread()?,
        false => server.start_singlethread()?,
    }
    Ok(())
}
