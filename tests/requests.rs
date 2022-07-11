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
        http_protocol_version: HttpProtocolVersion::OneDotOne, 
        security_headers: false,
        append_extra_headers: false,
        extra_headers: vec![],
    };
    let server = Server::new(configuration)?;
    server.serve_request(&mut None, body)?;
}

#[cfg(test)]
mod tests {
    use vrs::enums::error::ServerError;
    use vrs::enums::http::HttpProtocolVersion;
    use vrs::structs::configuration::Configuration;
    use vrs::structs::server::Server;
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
            append_extra_headers: false,
            extra_headers: vec![],
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

    /// Given a HTTP response, returns the status code.
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

