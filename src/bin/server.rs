#![deny(unsafe_code)]

use std::sync::Arc;

use vrs::configuration::*;
use vrs::enums::error::ServerError;
use vrs::structs::configuration::Configuration;
use vrs::structs::server::Server;

pub fn main() -> Result<(), ServerError> {
    /* Stock configuration for VRS. Globals should not be used in test cases. */
    let config = Configuration {
        absolute_static_content_path: ABSOLUTE_STATIC_CONTENT_PATH,
        absolute_logs_path: ABSOLUTE_LOGS_PATH,
        save_logs: SAVE_LOGS,
        addr: ADDR,
        port: PORT,
        multithreading: MULTITHREADING,
        num_of_threads: NUM_OF_THREADS,
        http_protocol_version: HTTP_PROTOCOL_VERSION,
        allowed_methods: ALLOWED_METHODS.to_vec(),
        allow_all_origins: ALLOW_ALL_ORIGINS,
        allowed_origins: ALLOWED_ORIGINS.to_vec(),
        security_headers: SECURITY_HEADERS,
        allow_iframes: ALLOW_IFRAMES,
    };

    let server = Arc::new(Server::new(config.clone())?);

    match MULTITHREADING {
        true => server.start_multithread()?,
        false => server.start_singlethread()?,
    }
    Ok(())
}
