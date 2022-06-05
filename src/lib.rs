mod core;
pub mod structs;

use crate::core::server::start_server;
use structs::config::Config;

pub fn init_builder(config: Config) {
    let mut final_config = Config {
        addr: Some("127.0.0.1".to_string()),
        port: Some(80),
        debug_info: false,
    };

    if config.addr != None {
        final_config.addr = config.addr;
    }
    if config.port != None {
        final_config.port = config.port;
    }
    if config.debug_info {
        final_config.debug_info = true;
    }

    start_server(
        final_config.addr.unwrap(),
        final_config.port.unwrap(),
        final_config.debug_info,
    )
}

pub fn init() {
    start_server("0.0.0.0".to_string(), 80, false)
}
