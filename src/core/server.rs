use super::configuration::Configuration;
use super::cors::Cors;
use super::socket::{parse_utf8, read_stream};
use super::uri::URI;
use crate::error::ServerError;
use crate::file::{get_file_extension, CachedFile};
use crate::license::print_license_info;
use crate::response::factory::ResponseFactory;
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
use std::sync::Arc;
use std::{fs::OpenOptions, net::TcpListener};

pub struct Server<'a> {
    unixtime: u64,
    cors: Cors<'a>,
    config: Configuration<'a>,
}

impl<'a> Server<'a> {
    pub fn new(config: Configuration<'a>) -> std::io::Result<Self> {
        let unixtime = generate_unixtime().expect("Failed generating system unix time");

        let config_ref = config.clone();

        Ok(Self {
            unixtime,
            cors: Cors::new(
                config_ref.allowed_origins,
                config_ref.allow_all_origins,
                config_ref.allowed_methods,
            )?,
            config,
        })
    }

    pub fn start_multithread(self: Arc<Self>) -> Result<(), ServerError>
    where
        Self: 'static,
    {
        if self.config.print_license_info {
            print_license_info();
        }

        let listener = TcpListener::bind(
            [self.config.addr, ":", self.config.port.to_string().as_str()].concat(),
        )?;

        if !self.config.use_security_headers {
            println!("Production note: security headers are currently turned off, keep it enabled in production!")
        }

        let pool = ThreadPool::new(self.config.num_of_threads)?;

        let mut state = AppState::default();

        if self.config.cache_files {
            state.cached_files = Some(HashMap::new());
        }

        let (cache_tx, cache_rx): (
            Sender<HashMap<String, CachedFile>>,
            Receiver<HashMap<String, CachedFile>>,
        ) = mpsc::channel();

        for stream in listener.incoming() {
            let self_ref = Arc::clone(&self);
            let state_ref = state.clone();
            let cache_tx_ref = cache_tx.clone();

            pool.execute(|| {
                let self_ref = self_ref;

                let mut stream = stream.unwrap();
                let mut state_ref = state_ref;
                let tx = cache_tx_ref;

                let response = Self::serve_request(&self_ref, None, &mut stream, &mut state_ref);

                // Note: `.unwrap()` will only make one of the threads panic in multithreaded mode, so unwrapping instead of returning the error is fine.
                stream.write_all(&response.unwrap().as_bytes()).unwrap();
                stream.flush().unwrap();

                if self_ref.config.cache_files {
                    tx.send(state_ref.cached_files.expect(
                        "'cached_files' is None even though 'cache_files' boolean is set to true.",
                    ))
                    .expect("Failed transmitting app's state.");
                }
            });

            if self.config.cache_files {
                state.cached_files = Some(cache_rx.recv().unwrap());
            }
        }

        Ok(())
    }

    pub fn start_singlethread(&self) -> Result<(), ServerError> {
        if self.config.print_license_info {
            print_license_info();
        }

        let listener = TcpListener::bind(
            [self.config.addr, ":", self.config.port.to_string().as_str()].concat(),
        )?;

        /* Create the log file and return error if it fails creating or opening existing one */
        let mut logfile = if self.config.save_logs {
            let result = Some(
                OpenOptions::new().append(true).create(true).open(
                    [
                        self.config.absolute_logs_path,
                        "/",
                        self.unixtime.to_string().as_str(),
                    ]
                    .concat(),
                ),
            );
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

        if !self.config.use_security_headers {
            println!("Production note: security headers are currently turned off, keep it enabled in production!");
        }

        let mut state = AppState::default();

        if self.config.cache_files {
            state.cached_files = Some(HashMap::new());
        }

        for stream in listener.incoming() {
            let mut stream = stream?; /* Note that stream is a result. */

            let response = self.serve_request(logfile.as_mut(), &stream, &mut state)?;

            stream.write_all(&response.as_bytes())?;
            stream.flush()?;
        }

        Ok(())
    }

    pub fn serve_request(
        &self,
        logfile: Option<&mut File>,
        input: impl Read,
        state: &mut AppState,
    ) -> Result<String, ServerError> {
        let final_response = FinalResponse::special_default_builder(&self.config);

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
            if self.config.allow_all_origins {
                "*".to_string()
            } else if self.cors.origin_is_allowed(&origin) {
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

        if !self.cors.method_is_allowed(&buf_utf8) {
            return final_response.status_code(405).build();
        }

        let mut uri = URI::new();

        uri.find(&buf_utf8);

        let uri_path_ref = uri.get().as_ref();

        let uri_path_ref = match uri_path_ref {
            Some(path) => path,
            None => return final_response.status_code(400).build(),
        };

        let absolute_path = [self.config.absolute_static_content_path, "/", uri_path_ref].concat();

        let path = Path::new(&absolute_path);

        if path.is_dir() {
            if self.config.allow_directory_listing {
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

        let file_ext = get_file_extension(uri_path_ref.as_str());

        if self.config.cache_files {
            match &mut state.cached_files {
                Some(ref mut cached_files) => {
                    if let Some(cached_file) = cached_files.get(uri_path_ref.as_str()) {
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
                            fs::File::open([self.config.absolute_static_content_path, "/", uri_path_ref].concat());

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

                        cached_files.insert(uri_path_ref.clone(), CachedFile {
                            extension: file_ext.to_string(),
                            content: requested_content.clone(),
                        });

                        return final_response
                            .response_type(
                                ResponseType::File(
                                    FileResponse {
                                        file_ext: file_ext,
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
            fs::File::open([self.config.absolute_static_content_path, "/", uri_path_ref].concat());

        let mut requested_file = match requested_file {
            Ok(file) => file,
            Err(_err) => return final_response.status_code(404).build(),
        };

        let mut requested_content = String::new();

        if let Err(_) = requested_file.read_to_string(&mut requested_content) {
            return Err(ServerError::IOError(io::Error::new(
                io::ErrorKind::InvalidData,
                "File is not valid UTF-8 data.",
            )));
        }

        final_response
            .response_type(ResponseType::File(FileResponse {
                file_ext: file_ext,
                file_content: requested_content.as_str(),
            }))
            .build()
    }
}
