mod configuration;
mod enums;
mod structs;
mod util;

use std::io::ErrorKind;

use structs::server::Server;

pub fn main() -> std::io::Result<()> {
    let mut server = Server::new();

    server.start()?;
    Ok(())
}
