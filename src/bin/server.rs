#![deny(unsafe_code)]

use vrs::configuration::*;
use vrs::core::configuration::Configuration;
use vrs::core::server;
use vrs::error::ServerError;

pub fn main() -> Result<(), ServerError> {
    let config = Configuration::read_from_vars();

    match MULTITHREADING {
        true => server::start_multithread(config)?,
        false => server::start_singlethread(config)?,
    }

    Ok(())
}
