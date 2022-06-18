use crate::configuration::*;
use crate::enums::server::StatusCode;
use crate::util::headers::Header;
use crate::util::license::print_license_info;
use crate::util::response::{handle_response, ServerResponse};
use crate::util::socket::{parse_utf8, read_stream};
use crate::util::time::generate_unixtime;

use std::fs::{self, File};
use std::io::Write;
use std::net::TcpStream;
use std::{fs::OpenOptions, net::TcpListener};

use super::cors::Cors;
use super::uri::URI;

pub struct Server<'a> {
    unixtime: u64,
    cors: Cors<'a>,
}

impl Server<'_> {
    pub fn new() -> Self {
        let unixtime = generate_unixtime().expect("Failed generating system unix time");

        Self {
            unixtime: unixtime,
            cors: Cors::new(),
        }
    }

    pub fn start(&mut self) {
        print_license_info();

        let listener = TcpListener::bind(format!("{ADDR}:{PORT}")).unwrap();

        let logfile = if SAVE_LOGS {
            Some(
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open([ABSOLUTE_LOGS_PATH, "/", self.unixtime.to_string().as_str()].concat())
                    .expect("Failed to create logfile"),
            )
        } else {
            None
        };

        if !SECURITY_HEADERS {
            println!("Production note: security headers are currently turned off, keep it enabled in production!")
        }

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            if MULTITHREADING {
                panic!("Multithreading is currently disabled for future rewrite.");
            } else {
                let handled = self.singlethread_handle_connection(&logfile, &mut stream);

                let response = handle_response(handled);

                stream.write(&response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }

    fn multithread_handle_connection<'a>(&self, stream: &'a mut TcpStream) -> ServerResponse<'a> {
        let (mut req_headers, buf) = read_stream(stream)?;

        let buf_utf8 = parse_utf8(&req_headers, &buf)?;

        let origin = match req_headers.get("Origin") {
            Some(header) => header.to_string(),
            None => "null".to_string(),
        };

        req_headers
            .insert(
                "Access-Control-Allow-Origin".to_string(),
                if ALLOW_ALL_ORIGINS {
                    "*".to_string()
                } else if self.cors.origin_is_allowed(&origin) {
                    origin.to_string()
                } else {
                    "null".to_string()
                },
            )
            .unwrap();

        self.main_logic(req_headers, buf_utf8)
    }

    fn singlethread_handle_connection<'a>(
        &self,
        logfile: &'a Option<File>,
        stream: &'a mut TcpStream,
    ) -> ServerResponse<'a> {
        let (mut req_headers, buf) = read_stream(stream)?;

        let origin = match req_headers.get("Origin") {
            Some(header) => header.to_string(),
            None => "null".to_string(),
        };

        req_headers.insert(
            "Access-Control-Allow-Origin".to_string(),
            if ALLOW_ALL_ORIGINS {
                "*".to_string()
            } else if self.cors.origin_is_allowed(&origin) {
                origin.to_string()
            } else {
                "null".to_string()
            },
        );

        let buf_utf8 = parse_utf8(&req_headers, &buf)?;

        self.main_logic(req_headers, buf_utf8)
    }

    fn main_logic<'a>(&self, req_headers: Header, buf_utf8: String) -> ServerResponse<'a> {
        if !self.cors.method_is_allowed(&buf_utf8) {
            return Ok((req_headers, StatusCode::MethodNotAllowed, None));
        }

        let mut uri = URI::new();

        uri.find(&buf_utf8);

        if uri.get() == &None {
            return Ok((req_headers, StatusCode::BadRequest, None));
        };

        let requested_content = fs::File::open(format!(
            "{ABSOLUTE_STATIC_CONTENT_PATH}/{}",
            uri.get().clone().unwrap()
        ));
        let response = match requested_content {
            Ok(file) => file,
            Err(_err) => {
                return Ok((req_headers, StatusCode::NotFound, None));
            }
        };

        Ok((req_headers.clone(), StatusCode::OK, Some(response)))
    }
}
