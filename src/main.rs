mod configuration;
mod constants;
mod core;
mod structs;
mod util;

use crate::core::server::start_server;
use crate::core::time::generate_unixtime;

pub fn main() {
    let unixtime = generate_unixtime();

    start_server(unixtime)
}
