use crate::configuration::*;
use crate::util::file::get_file_extension;
use crate::util::headers::Header;
use crate::util::license::print_license_info;
use crate::util::response::{handle_response, ServerResponse};
use crate::util::socket::{parse_utf8, read_stream};
use crate::util::time::generate_unixtime;

use std::fs::{self, File};
use std::io::Write;

use std::net::TcpStream;
use std::sync::Arc;

use std::{fs::OpenOptions, net::TcpListener};

use super::cors::Cors;
use super::thread::ThreadPool;
use super::uri::URI;

pub struct Server {
    unixtime: u64,
    cors: Cors<'static>,
}

impl Server {
    pub fn new() -> std::io::Result<Self> {
        let unixtime = generate_unixtime().expect("Failed generating system unix time");

        Ok(Self {
            unixtime,
            cors: Cors::new()?,
        })
    }

    pub fn start_multithread(self: Arc<Self>) -> std::io::Result<()> {
        print_license_info();

        let listener = TcpListener::bind(format!("{ADDR}:{PORT}")).unwrap();

        if !SECURITY_HEADERS {
            println!("Production note: security headers are currently turned off, keep it enabled in production!");
        }

        let pool = ThreadPool::new(NUM_OF_THREADS).unwrap();

        for stream in listener.incoming() {
            let self_ref = Arc::clone(&self);

            pool.execute(move || {
                let mut stream = stream.unwrap();
                let handled = Self::multithread_handle_connection(&self_ref, &mut stream);

                /* .handle_response() will handles the client errors */
                let response = handle_response(handled);

                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            });
        }

        Ok(())
    }

    pub fn start_singlethread(&self) -> Result<(), std::io::Error> {
        print_license_info();

        let listener = TcpListener::bind(format!("{ADDR}:{PORT}")).unwrap();

        /* Create the log file and return error if it fails creating or opening existing one */
        let mut logfile = if SAVE_LOGS {
            let result = Some(
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open([ABSOLUTE_LOGS_PATH, "/", self.unixtime.to_string().as_str()].concat()),
            );
            if let Ok(file) = result.expect("Something went wrong whilst unwrapping the logfile.") {
                Some(file)
            } else {
                println!("Warning: Failed creating or opening logfile. Logs will not be saved.");
                None
            }
        } else {
            None
        };

        if !SECURITY_HEADERS {
            println!("Production note: security headers are currently turned off, keep it enabled in production!");
        }

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let handled = self.singlethread_handle_connection(&mut logfile, &mut stream);

            /* .handle_response() will handles the client errors */
            let response = handle_response(handled);

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        Ok(())
    }

    fn multithread_handle_connection(&self, stream: &mut TcpStream) -> ServerResponse {
        let (mut req_headers, buf) = read_stream(stream)?;

        let buf_utf8 = parse_utf8(&req_headers, &buf)?;

        let origin = match req_headers.get("Origin") {
            Some(header) => header.to_string(),
            None => "null".to_string(),
        };

        req_headers.insert(
            "Access-Control-Allow-Origin".to_string(),
            if ALLOW_ALL_ORIGINS {
                "*".to_string()
            } else if self.cors.origin_is_allowed(&origin) {
                origin
            } else {
                "null".to_string()
            },
        );

        self.main_logic(req_headers, &buf_utf8)
    }

    fn singlethread_handle_connection(
        &self,
        logfile: &mut Option<File>,
        stream: &mut TcpStream,
    ) -> ServerResponse {
        let (mut req_headers, buf) = read_stream(stream)?;

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
TIME: {}
                    ",
                        req_headers,
                        generate_unixtime().unwrap()
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
            if ALLOW_ALL_ORIGINS {
                "*".to_string()
            } else if self.cors.origin_is_allowed(&origin) {
                origin
            } else {
                "null".to_string()
            },
        );

        let buf_utf8 = parse_utf8(&req_headers, &buf)?;

        self.main_logic(req_headers, &buf_utf8)
    }

    fn main_logic<'a>(&self, req_headers: Header, buf_utf8: &str) -> ServerResponse<'a> {
        if !self.cors.method_is_allowed(buf_utf8) {
            return Err((req_headers, 405));
        }

        let mut uri = URI::new();

        uri.find(buf_utf8);

        if uri.get() == &None {
            return Err((req_headers, 400));
        };

        let requested_content = fs::File::open(format!(
            "{ABSOLUTE_STATIC_CONTENT_PATH}/{}",
            uri.get().clone().unwrap()
        ));
        let response = match requested_content {
            Ok(file) => file,
            Err(_err) => {
                return Err((req_headers, 404));
            }
        };

        Ok((
            req_headers,
            Some(get_file_extension(uri.get().clone().unwrap().as_str()).to_string()),
            Some(response),
        ))
    }
}
