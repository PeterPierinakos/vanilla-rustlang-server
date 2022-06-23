use crate::configuration::ABSOLUTE_STATIC_CONTENT_PATH;
use crate::util::file::get_file_extension;
use crate::util::license::print_license_info;
use crate::util::response::{create_dir_response, create_file_response};
use crate::util::socket::{parse_utf8, read_stream};
use crate::util::time::generate_unixtime;

use std::fs::{self, File};
use std::io::Write;

use std::net::TcpStream;
use std::path::Path;
use std::sync::Arc;

use std::{fs::OpenOptions, net::TcpListener};

use super::configuration::Configuration;
use super::cors::Cors;
use super::thread::ThreadPool;
use super::uri::URI;

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
            unixtime: unixtime,
            cors: Cors::new(
                config_ref.allowed_origins,
                config_ref.allow_all_origins,
                config_ref.allowed_methods,
            )?,
            config: config,
        })
    }

    pub fn start_multithread(self: Arc<Self>) -> std::io::Result<()>
    where
        Self: 'static,
    {
        print_license_info();

        let listener = TcpListener::bind(
            [self.config.addr, ":", self.config.port.to_string().as_str()].concat(),
        )?;

        if !self.config.security_headers {
            println!("Production note: security headers are currently turned off, keep it enabled in production!")
        }

        let pool = ThreadPool::new(self.config.num_of_threads).unwrap();

        for stream in listener.incoming() {
            let self_ref = Arc::clone(&self);

            pool.execute(move || {
                let mut stream = stream.unwrap();
                let response = Self::serve_request(&self_ref, &mut None, &mut stream);

                // The thread currently panics if the response returns an error. TODO: fix the possible error.
                stream.write_all(&response.unwrap().as_bytes()).unwrap();
                stream.flush().unwrap();
            });
        }

        Ok(())
    }

    pub fn start_singlethread(&self) -> Result<(), std::io::Error> {
        print_license_info();

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

        if !self.config.security_headers {
            println!("Production note: security headers are currently turned off, keep it enabled in production!");
        }

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let response = self.serve_request(&mut logfile, &mut stream)?;

            stream.write(&response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        Ok(())
    }

    fn serve_request(
        &self,
        logfile: &mut Option<File>,
        stream: &mut TcpStream,
    ) -> std::io::Result<String> {
        let (mut req_headers, buf) = match read_stream(stream) {
            Ok((headers, buf)) => (headers, buf),
            Err((headers, status)) => return create_file_response(headers, None, None, status),
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
            Err((headers, status)) => create_file_response(headers, None, None, status)?,
        };

        if !self.cors.method_is_allowed(&buf_utf8) {
            return create_file_response(req_headers, None, None, 405);
        }

        let mut uri = URI::new();

        uri.find(&buf_utf8);

        let uri_path_ref = uri.get().as_ref();

        let uri_path_ref = match uri_path_ref {
            Some(path) => path,
            None => return create_file_response(req_headers, None, None, 400),
        };

        let absolute_path = [self.config.absolute_static_content_path, "/", uri_path_ref].concat();

        let path = Path::new(&absolute_path);

        if path.is_dir() {
            let path_iter = match path.read_dir() {
                Ok(iter) => iter,
                Err(_) => return create_file_response(req_headers, None, None, 500),
            };
            return create_dir_response(req_headers, path_iter);
        }

        let requested_content =
            fs::File::open([ABSOLUTE_STATIC_CONTENT_PATH, "/", uri_path_ref].concat());

        let response = match requested_content {
            Ok(file) => file,
            Err(_err) => return create_file_response(req_headers, None, None, 404),
        };

        create_file_response(
            req_headers.clone(),
            Some(get_file_extension(uri.get().clone().unwrap().as_str()).to_string()),
            Some(response),
            200,
        )
    }
}
