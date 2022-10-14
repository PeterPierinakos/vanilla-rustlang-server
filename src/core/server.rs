use super::configuration::Configuration;
use super::socket::{parse_utf8, read_stream};
use super::uri::*;
use crate::compile_if_eq;
use crate::error::ServerError;
use crate::file::{get_file_extension, CachedFile};
use crate::response::{final_response::FinalResponse, types::*};
use crate::state::AppState;
use crate::thread::ThreadPool;
use crate::time::generate_unixtime;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{fs::OpenOptions, net::TcpListener};

/// Function executed during server initialization for initial server tasks such as printing the software license's information.
fn do_initial_tasks(config: &Configuration) {
    compile_if_eq!(config.print_license_info_at_start, true, {
        crate::license_info!();
    });
}

/// The main initializer for the server used by the single-threaded and multi-threaded initializers.
///
/// The `init` closure additionally takes a `ThreadPool` which can be disregarded if the server isn't planning to use multiple threads.
fn server_initializer<F: FnOnce(ThreadPool, TcpListener, AppState) -> Result<(), ServerError>>(
    config: &Configuration,
    init: F,
) -> Result<(), ServerError> {
    let pool = ThreadPool::new(config.num_of_threads)?;

    do_initial_tasks(config);

    let listener =
        TcpListener::bind(format!("{}:{}", config.addr, config.port))?;

    let state = AppState::default();

    init(pool, listener, state)
}

pub fn start_multithread(config: Configuration) -> Result<(), ServerError> {
    server_initializer(&config, |pool, listener, mut state| {
        if !config.use_security_headers {
            println!("Production note: security headers are currently turned off, keep it enabled in production!")
        }

        if config.cache_files {
            state.cached_files = Some(HashMap::new());
        }

        let (cache_tx, cache_rx): (
            Sender<HashMap<String, CachedFile>>,
            Receiver<HashMap<String, CachedFile>>,
        ) = mpsc::channel();

        for stream in listener.incoming() {
            let state_ref = state.clone();
            let cache_tx_ref = cache_tx.clone();

            pool.execute(|| {
                let config = Configuration::test_config();

                let mut stream = stream.unwrap();
                let mut state_ref = state_ref;
                let tx = cache_tx_ref;

                let response = serve_request(&config, None, &mut stream, &mut state_ref);

                // Note: `.unwrap()` will only make one of the threads panic in multithreaded mode, so unwrapping instead of returning the error is fine.
                stream.write_all(&response.unwrap().as_bytes()).unwrap();
                stream.flush().unwrap();

                if config.cache_files {
                    tx.send(state_ref.cached_files.expect(
                        "'cached_files' is None even though 'cache_files' boolean is set to true.",
                    ))
                    .expect("Failed transmitting app's state.");
                }
            });

            if config.cache_files {
                state.cached_files = Some(cache_rx.recv().unwrap());
            }
        }

        Ok(())
    })
}

pub fn start_singlethread(config: Configuration) -> Result<(), ServerError> {
    server_initializer(&config, |_, listener, mut state| {
        let unix_ts = generate_unixtime()?;

        /* Create the log file and return error if it fails creating or opening existing one */
        let mut logfile =
            if config.save_logs {
                let result =
                    Some(OpenOptions::new().append(true).create(true).open(
                        [config.absolute_logs_path, "/", unix_ts.to_string().as_str()].concat(),
                    ));
                match result.expect("Something went wrong whilst unwrapping the logfile.") {
                    Ok(file) => Some(file),
                    Err(_) => {
                        println!(
                            "Warning: Failed creating or opening logfile. Logs will not be saved."
                        );
                        None
                    }
                }
            } else {
                None
            };

        if !config.use_security_headers {
            println!("Production note: security headers are currently turned off, keep it enabled in production!");
        }

        if config.cache_files {
            state.cached_files = Some(HashMap::new());
        }

        for stream in listener.incoming() {
            let mut stream = stream?; /* Note that stream is a result. */

            let response = serve_request(&config, logfile.as_mut(), &stream, &mut state)?;

            stream.write_all(&response.as_bytes())?;
            stream.flush()?;
        }

        Ok(())
    })
}

pub fn serve_request(
    config: &Configuration,
    logfile: Option<&mut File>,
    input: impl Read,
    state: &mut AppState,
) -> Result<String, ServerError> {
    let final_response = FinalResponse {
        status_code: 200,
        req_headers: None,
        response_type: None,
        config,
    };

    // Default to fallback response since it's the most common.
    let final_response = final_response.response_type(ResponseType::Fallback);

    let (mut req_headers, buf) = match read_stream(input) {
        Ok((headers, buf)) => (headers, buf),
        Err(status) => {
            return final_response
                .req_headers(HashMap::new())
                .status_code(status)
                .build();
        }
    };

    let origin = match req_headers.get("Origin") {
        Some(header) => header.to_string(),
        None => "null".to_string(),
    };

    match logfile {
        Some(file) => {
            match file.write_all(
                format!(
                    "
-- NEW REQUEST --
HEADERS: {:?}
                    ",
                    req_headers,
                )
                .as_bytes(),
            ) {
                Ok(()) => {}
                Err(_) => {
                    println!("Warning: something went wrong whilst writing to the logfile. Maybe it's too large?");
                }
            }
        }
        None => {}
    }

    req_headers.insert(
        "Access-Control-Allow-Origin".to_string(),
        if config.allow_all_origins {
            "*".to_string()
        } else if config.allowed_origins.contains(origin.as_str()) {
            origin
        } else {
            "null".to_string()
        },
    );

    let buf_utf8 = match parse_utf8(&req_headers, &buf) {
        Ok(utf8) => utf8,
        Err((headers, status)) => {
            for (key, val) in headers {
                req_headers.insert(key.to_string(), val.to_string());
            }
            return final_response
                .req_headers(req_headers)
                .status_code(status)
                .build();
        }
    };

    let final_response = final_response.req_headers(req_headers);

    let req_method = match buf_utf8.split_whitespace().next() {
        Some(req_method) => req_method,
        None => return final_response.status_code(500).build(),
    };

    if !config.allowed_methods.contains(req_method) {
        return final_response.status_code(405).build();
    }

    let urn = match find_urn(&buf_utf8) {
        Some(urn) => urn,
        None => return final_response.status_code(400).build(),
    };

    let absolute_path = format!("{}/{urn}", config.absolute_static_content_path);

    let path = Path::new(&absolute_path);

    if path.is_dir() {
        if config.allow_directory_listing {
            let path_iterator = match path.read_dir() {
                Ok(iter) => iter,
                Err(_) => return final_response.status_code(500).build(),
            };
            return final_response
                .response_type(ResponseType::Dir(DirResponse { path_iterator }))
                .build();
        } else {
            return final_response.status_code(404).build();
        }
    }

    let file_ext = get_file_extension(&urn);

    if config.cache_files {
        match &mut state.cached_files {
                Some(ref mut cached_files) => {
                    if let Some(cached_file) = cached_files.get(&urn) {
                        return final_response
                            .response_type(
                                ResponseType::File(
                                    FileResponse {
                                        file_ext: cached_file.extension.as_str(),
                                        file_content: cached_file.content.as_str(),
                                    }
                                )
                            )
                            .build()
                    }
                    else {
                        let requested_file =
                            fs::File::open(format!("{}/{}", config.absolute_static_content_path, &urn));

                        let mut requested_file = match requested_file {
                            Ok(file) => file,
                            Err(_err) => return final_response.status_code(404).build()
                        };

                        let mut requested_content = String::new();

                        if let Err(_) = requested_file.read_to_string(&mut requested_content) {
                            return Err(ServerError::IOError(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "File is not valid UTF-8 data.",
                            )));
                        }

                        cached_files.insert(urn.to_string(), CachedFile {
                            extension: file_ext.to_string(),
                            content: requested_content.clone(),
                        });

                        return final_response
                            .response_type(
                                ResponseType::File(
                                    FileResponse {
                                        file_ext,
                                        file_content: requested_content.as_str(),
                                    }
                                )
                            )
                            .build()
                    }
                },
                None => return Err(ServerError::from(io::Error::new(io::ErrorKind::Other, "State is a None value even though 'cache_files' configuration is set to true. This should never occur, this is probably a bug.")))
            };
    }

    let requested_file =
        fs::File::open(format!("{}/{urn}", config.absolute_static_content_path));

    let mut requested_file = match requested_file {
        Ok(file) => file,
        Err(_err) => return final_response.status_code(404).build(),
    };

    let mut requested_content = String::new();

    if requested_file.read_to_string(&mut requested_content).is_err() {
        return Err(ServerError::IOError(io::Error::new(
            io::ErrorKind::InvalidData,
            "File is not valid UTF-8 data.",
        )));
    }

    final_response
        .response_type(ResponseType::File(FileResponse {
            file_ext,
            file_content: requested_content.as_str(),
        }))
        .build()
}
