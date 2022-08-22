#![deny(unsafe_code)]

use std::sync::Arc;

use vrs::configuration::*;
use vrs::core::configuration::Configuration;
use vrs::core::server::Server;
use vrs::error::ServerError;

pub fn main() -> Result<(), ServerError> {
    /* Stock configuration for VRS. Globals should not be used in test cases. */
    let config = Configuration::read_from_vars();

    let server = Arc::new(Server::new(config.clone())?);

    match MULTITHREADING {
        true => server.start_multithread()?,
        false => server.start_singlethread()?,
    }

    Ok(())
}
