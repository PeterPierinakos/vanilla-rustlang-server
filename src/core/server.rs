use super::configuration::Configuration;
use std::collections::HashMap;
use crate::response::factory::ResponseFactory;
use super::cors::Cors;
use super::socket::{parse_utf8, read_stream};
use super::uri::URI;
use crate::error::ServerError;
use crate::file::get_file_extension;
use crate::license::print_license_info;
use crate::response::{types::*, final_response::FinalResponse};
use crate::thread::ThreadPool;
use crate::time::generate_unixtime;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
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

        for stream in listener.incoming() {
            let self_ref = Arc::clone(&self);

            pool.execute(move || {
                let mut stream = stream.unwrap();
                let response = Self::serve_request(&self_ref, &mut None, &mut stream);

                // Note: `.unwrap()` will only make one of the threads panic in multithreaded mode, so unwrapping instead of returning the error is fine.
                stream.write_all(&response.unwrap().as_bytes()).unwrap();
                stream.flush().unwrap();
            });
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

        for stream in listener.incoming() {
            let mut stream = stream?; /* Note that stream is a result. */

            let response = self.serve_request(&mut logfile, &stream)?;

            stream.write_all(&response.as_bytes())?;
            stream.flush()?;
        }

        Ok(())
    }

    pub fn serve_request(
        &self,
        logfile: &mut Option<File>,
        input: impl Read,
    ) -> Result<String, ServerError> {
        let final_response = FinalResponse::special_default_builder(&self.config);

        // Default to fallback response since it's the most common.
        let final_response = final_response.response_type(ResponseType::Fallback);

        let (mut req_headers, buf) = match read_stream(input) {
            Ok((headers, buf)) => (headers, buf),
            Err(status) => {
                return final_response.req_headers(HashMap::new()).status_code(status).build();
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
                origin.to_string()
            } else {
                "null".to_string()
            },
        );

        let buf_utf8 = match parse_utf8(&req_headers, &buf) {
            Ok(utf8) => utf8,
            Err((headers, status)) => {
                for (key, val) in headers {
                    req_headers.insert(key, val);
                }
                return final_response.req_headers(req_headers).status_code(status).build()
            }
        };

        let final_response = final_response.req_headers(req_headers);

        if !self.cors.method_is_allowed(&buf_utf8) {
            return final_response.status_code(405).build()
        }

        let mut uri = URI::new();

        uri.find(&buf_utf8);

        let uri_path_ref = uri.get().as_ref();

        let uri_path_ref = match uri_path_ref {
            Some(path) => path,
            None => return final_response.status_code(400).build()
        };

        let absolute_path = [self.config.absolute_static_content_path, "/", uri_path_ref].concat();

        let path = Path::new(&absolute_path);

        if path.is_dir() {
            if self.config.allow_directory_listing {
                let path_iterator = match path.read_dir() {
                    Ok(iter) => iter,
                    Err(_) => {
                        return final_response.status_code(500).build()
                    }
                };
                return final_response.response_type(ResponseType::Dir(DirResponse {
                    path_iterator,
                })).build()
            } else {
                return final_response.status_code(404).build()
            }
        }

        let requested_content =
            fs::File::open([self.config.absolute_static_content_path, "/", uri_path_ref].concat());

        let requested_content = match requested_content {
            Ok(file) => file,
            Err(_err) => return final_response.status_code(404).build()
        };

        final_response
            .response_type(
                ResponseType::File(
                    FileResponse {
                        file_ext: get_file_extension(uri.get().clone().unwrap().as_str()).to_string(),
                        file: requested_content,
                    }
                )
            )
            .build()
    }
}
