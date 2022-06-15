mod configuration;
mod enums;
mod structs;
mod util;

use crate::util::server::start_server;
use crate::util::time::generate_unixtime;

pub fn main() {
    let unixtime = generate_unixtime();

    start_server(unixtime)
}
