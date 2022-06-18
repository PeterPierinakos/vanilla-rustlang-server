mod configuration;
mod enums;
mod structs;
mod util;

use structs::server::Server;

pub fn main() {
    let mut server = Server::new();
    server.start()
}
