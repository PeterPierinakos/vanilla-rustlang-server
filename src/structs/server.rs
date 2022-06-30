use crate::enums::error::ServerError;
use crate::util::file::get_file_extension;
use crate::util::license::print_license_info;
use crate::util::response::{create_dir_response, create_file_response};
use crate::util::socket::{parse_utf8, read_stream};
use crate::util::time::generate_unixtime;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};

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

    pub fn start_multithread(self: Arc<Self>) -> Result<(), ServerError>
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
            let mut stream = stream?; /* Note that stream is a result. */

            let response = self.serve_request(&mut logfile, &stream)?;

            stream.write_all(&response.as_bytes())?;
            stream.flush()?;
        }

        Ok(())
    }

    fn serve_request(
        &self,
        logfile: &mut Option<File>,
        input: impl Read,
    ) -> Result<String, ServerError> {
        let (mut req_headers, buf) = match read_stream(input) {
            Ok((headers, buf)) => (headers, buf),
            Err(status) => return Ok(create_file_response(HashMap::new(), None, None, status)?),
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
            fs::File::open([self.config.absolute_static_content_path, "/", uri_path_ref].concat());

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

/// This function only compiles when fuzzing, and exposes the server without actually opening and connecting to a port.
#[cfg(fuzzing)]
pub fn fuzz_serve_request(body: &[u8]) {
    let logs_path = std::env::temp_dir()
        .canonicalize()?
        .to_string_lossy()
        .into_owned();
    let configuration = Configuration {
        absolute_logs_path: &logs_path,
        absolute_static_content_path: concat!(env!("CARGO_MANIFEST_DIR"), "/", "tests/static"),
        addr: "localhost",
        // Setting the port to 0 takes advantage of an OS behavior that
        // always uses a free port when assigned in this manner on all
        // major platforms.
        port: 0,
        allow_all_origins: false,
        allow_iframes: false,
        allowed_methods: vec!["GET"],
        allowed_origins: vec!["localhost"],
        save_logs: false,
        multithreading: false,
        num_of_threads: 1,
        http_protocol_version: crate::enums::http::HttpProtocolVersion::OneDotOne,
        security_headers: false,
    };
    let server = Server::new(configuration)?;
    server.serve_request(&mut None, body)?;
}

#[cfg(test)]
mod tests {
    use crate::enums::error::ServerError;
    use crate::enums::http::HttpProtocolVersion;
    use crate::structs::configuration::Configuration;
    use crate::structs::server::Server;
    use std::io::Cursor;

    // This is thread local to make it possible to get around the lack of a suitable once_cell in std.
    // Note that this will leak memory in each thread that references it, and is therefore not well-suited for many tasks.
    // In the future, this should be replaced with a OnceCell, or the Configuration struct should use Cow instead of &str.
    thread_local! {
        static LOGS_PATH: &'static str = Box::leak(
            std::env::temp_dir()
                .to_str()
                .expect("Temp dir path should be valid UTF-8")
                .to_string()
                .into_boxed_str(),
        );
    }

    fn create_test_server() -> Server<'static> {
        let logs_path = LOGS_PATH.with(|logs_path| *logs_path);
        Server::new(Configuration {
            absolute_logs_path: logs_path,
            absolute_static_content_path: concat!(env!("CARGO_MANIFEST_DIR"), "/", "tests/static"),
            addr: "localhost",
            // Setting the port to 0 takes advantage of an OS behavior that
            // always uses a free port when assigned in this manner on all
            // major platforms.
            port: 0,
            allow_all_origins: false,
            allow_iframes: false,
            allowed_methods: vec!["GET"],
            allowed_origins: vec!["localhost"],
            save_logs: false,
            multithreading: false,
            num_of_threads: 1,
            http_protocol_version: HttpProtocolVersion::OneDotOne,
            security_headers: false,
        })
        .expect("test server creation failed")
    }

    /// Create a request buffer for testing the server's core, with the arguments being stringly typed.
    /// info: the main part of the HTTP request (e.g. "GET / HTTP/1.1")
    /// headers: the headers of the HTTP requests (e.g. "Origin:localhost")
    fn create_test_buffer(info: &str, headers: Vec<&str>) -> Cursor<Vec<u8>> {
        let mut buf_utf8 = String::new();

        buf_utf8.push_str(info);
        buf_utf8.push('\n');

        for header in headers {
            buf_utf8.push_str(header);
            buf_utf8.push('\n');
        }

        Cursor::new(buf_utf8.into_bytes())
    }

    /// Given an response HTTP response, returns the status code.
    ///
    /// This function is simple and will panic on malformed HTTP.
    fn get_response_code(body: &str) -> Result<u16, ServerError> {
        let first_line = body.lines().next().expect("response body is empty");
        let response_code = first_line
            .split(' ')
            .nth(1)
            .expect("response body's first line is too short");
        let parsed = response_code.parse()?;
        Ok(parsed)
    }

    #[test]
    fn get_index_is_ok() -> Result<(), ServerError> {
        let test_server = create_test_server();
        let req = test_server.serve_request(
            &mut None,
            create_test_buffer(
                "GET / HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        )?;
        assert_eq!(get_response_code(&req)?, 200);

        Ok(())
    }

    #[test]
    fn no_headers_is_bad_request() -> Result<(), ServerError> {
        let test_server = create_test_server();
        let req =
            test_server.serve_request(&mut None, create_test_buffer("GET / HTTP/1.1", vec![]))?;
        assert_eq!(get_response_code(&req)?, 400);

        Ok(())
    }

    #[test]
    fn nonexistent_file_is_not_found() -> Result<(), ServerError> {
        let test_server = create_test_server();
        let req = test_server.serve_request(
            &mut None,
            create_test_buffer(
                "GET /notfound HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        )?;
        assert_eq!(get_response_code(&req)?, 404);

        Ok(())
    }

    #[test]
    fn path_traversal_is_bad_request() -> Result<(), ServerError> {
        let test_server = create_test_server();
        let req = test_server.serve_request(
            &mut None,
            create_test_buffer(
                "GET /../../../etc/passwd HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        )?;
        assert_eq!(get_response_code(&req)?, 400);

        Ok(())
    }

    #[test]
    fn forbidden_method_is_not_allowed() -> Result<(), ServerError> {
        let test_server = create_test_server();
        let req = test_server.serve_request(
            &mut None,
            create_test_buffer(
                "POST / HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        )?;
        assert_eq!(get_response_code(&req)?, 405);

        Ok(())
    }

    #[test]
    fn no_crash_on_empty_body() -> Result<(), ServerError> {
        let test_server = create_test_server();
        let res = test_server.serve_request(&mut None, Cursor::new([]))?;
        assert_eq!(get_response_code(&res)?, 400);

        Ok(())
    }

    #[test]
    fn no_crash_on_bad_unicode() -> Result<(), ServerError> {
        let test_server = create_test_server();
        let res = test_server.serve_request(&mut None, Cursor::new([0xc6]))?;
        assert_eq!(get_response_code(&res)?, 400);

        Ok(())
    }
}
