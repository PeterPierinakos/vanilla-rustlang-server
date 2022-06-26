use crate::configuration::ABSOLUTE_STATIC_CONTENT_PATH;
use crate::enums::status::TestStatusCode;
use crate::util::file::get_file_extension;
use crate::util::headers::find_buf_headers;
use crate::util::license::print_license_info;
use crate::util::response::{create_dir_response, create_file_response};
use crate::util::socket::{parse_utf8, read_stream};
use crate::util::time::generate_unixtime;

use std::collections::HashMap;
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

            stream.write_all(&response.as_bytes()).unwrap();
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
            Err(status) => return create_file_response(HashMap::new(), None, None, status),
        };

        println!("{:?}", buf);

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

pub struct TestServer<'a> {
    cors: Cors<'a>,
    config: Configuration<'a>,
}

impl<'a> TestServer<'a> {
    pub fn new(config: Configuration<'a>) -> std::io::Result<Self> {
        let config_ref = config.clone();

        Ok(Self {
            cors: Cors::new(
                config_ref.allowed_origins,
                config_ref.allow_all_origins,
                config_ref.allowed_methods,
            )?,
            config: config,
        })
    }

    /// Create a fake [u8; 1024] request buffer for testing the server's core, with the arguments being stringly typed.
    /// info: the main part of the HTTP request (e.g. "GET / HTTP/1.1")
    /// headers: the headers of the HTTP requests (e.g. "Origin:localhost")
    pub fn create_fake_buffer(info: &str, headers: Vec<&str>) -> [u8; 1024] {
        let mut fake_buf_utf8 = String::new();

        fake_buf_utf8.push_str(info);
        fake_buf_utf8.push('\n');

        for header in headers {
            fake_buf_utf8.push_str(header);
            fake_buf_utf8.push('\n');
        }

        let mut fake_buf: [u8; 1024] = [0; 1024];

        for (i, c) in fake_buf_utf8.as_bytes().iter().enumerate() {
            fake_buf[i] = *c;
        }

        fake_buf
    }

    pub fn serve_fake_request(
        &self,
        logfile: &mut Option<File>,
        buf: &[u8; 1024],
    ) -> TestStatusCode {
        let mut req_headers = match find_buf_headers(buf) {
            Ok(headers) => headers,
            Err(_) => return TestStatusCode::BadRequest,
        };

        let buf_utf8 = match parse_utf8(&req_headers, &buf) {
            Ok(utf8) => utf8,
            Err((headers, status)) => create_file_response(headers, None, None, status).unwrap(),
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

        if req_headers.get("Access-Control-Allow-Origin").unwrap() == "null" {
            return TestStatusCode::CORSError;
        }

        if !self.cors.method_is_allowed(&buf_utf8) {
            return TestStatusCode::MethodNotAllowed;
        }

        let mut uri = URI::new();

        uri.find(&buf_utf8);

        let uri_path_ref = uri.get().as_ref();

        let uri_path_ref = match uri_path_ref {
            Some(path) => path,
            None => return TestStatusCode::BadRequest,
        };

        let absolute_path = [self.config.absolute_static_content_path, "/", uri_path_ref].concat();

        let path = Path::new(&absolute_path);

        if path.is_dir() {
            return TestStatusCode::DirResponse;
        }

        let requested_content =
            fs::File::open([ABSOLUTE_STATIC_CONTENT_PATH, "/", uri_path_ref].concat());

        match requested_content {
            Ok(_) => (),
            Err(_err) => return TestStatusCode::NotFound,
        };

        TestStatusCode::OK
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::*;
    use crate::enums::status::TestStatusCode;
    use crate::structs::configuration::Configuration;
    use crate::structs::server::TestServer;

    fn create_test_server() -> TestServer<'static> {
        TestServer::new(Configuration {
            absolute_logs_path: ABSOLUTE_LOGS_PATH,
            absolute_static_content_path: ABSOLUTE_STATIC_CONTENT_PATH,
            addr: ADDR,
            port: PORT,
            allow_all_origins: false,
            allow_iframes: false,
            allowed_methods: vec!["GET"],
            allowed_origins: vec!["localhost"],
            save_logs: false,
            multithreading: false,
            num_of_threads: NUM_OF_THREADS,
            http_protocol_version: HTTP_PROTOCOL_VERSION,
            security_headers: false,
        })
        .expect("test server creation failed")
    }

    #[test]
    fn get_index_is_ok() {
        let test_server = create_test_server();
        let req = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer(
                "GET / HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        );
        assert_eq!(req, TestStatusCode::OK);
    }

    #[test]
    fn no_headers_is_bad_request() {
        let test_server = create_test_server();
        let req = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer("GET / HTTP/1.1", vec![]),
        );
        assert_eq!(req, TestStatusCode::BadRequest);
    }

    #[test]
    fn nonexistent_file_is_not_found() {
        let test_server = create_test_server();
        let req = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer(
                "GET /notfound HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        );
        assert_eq!(req, TestStatusCode::NotFound);
    }

    #[test]
    fn path_traversal_is_bad_request() {
        let test_server = create_test_server();
        let req = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer(
                "GET /../../../etc/passwd HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        );
        assert_eq!(req, TestStatusCode::BadRequest);
    }

    #[test]
    fn forbidden_origin_is_cors_error() {
        let test_server = create_test_server();
        let req = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer("GET / HTTP/1.1", vec!["User-Agent:rust"]),
        );
        assert_eq!(req, TestStatusCode::CORSError);
    }

    #[test]
    fn forbidden_method_is_not_allowed() {
        let test_server = create_test_server();
        let req = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer(
                "POST / HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        );
        assert_eq!(req, TestStatusCode::MethodNotAllowed);
    }
}
