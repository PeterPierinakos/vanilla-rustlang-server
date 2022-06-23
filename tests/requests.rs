#[cfg(test)]
mod tests {
    use vrs::configuration::*;
    use vrs::enums::status::TestStatusCode;
    use vrs::structs::configuration::Configuration;
    use vrs::structs::server::TestServer;

    #[test]
    fn test_responses() -> std::io::Result<()> {
        let test_server = TestServer::new(Configuration {
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
        })?;

        /* Should return OK because headers aren't empty and requested file exists */
        let test_1 = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer(
                "GET / HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        );

        /* Should return BadRequest because no headers are provided */
        let test_2 = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer("GET / HTTP/1.1", vec![]),
        );

        /* Should return NotFound because requested file does not exist */
        let test_3 = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer(
                "GET /notfound HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        );

        /* Should return BadRequest because requested file is outside of the static directory, hence a path traversal attack attempt */
        let test_4 = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer(
                "GET /../../../etc/passwd HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        );

        /* Should return CORSError because origin is not allowed */
        let test_5 = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer("GET / HTTP/1.1", vec!["User-Agent:rust"]),
        );

        /* Should return MethodNotAllowed because we have only specified "GET" as allowed */
        let test_6 = test_server.serve_fake_request(
            &mut None,
            &TestServer::create_fake_buffer(
                "POST / HTTP/1.1",
                vec!["User-Agent:rust", "Origin:localhost"],
            ),
        );

        assert_eq!(test_1, TestStatusCode::OK);
        assert_eq!(test_2, TestStatusCode::BadRequest);
        assert_eq!(test_3, TestStatusCode::NotFound);
        assert_eq!(test_4, TestStatusCode::BadRequest);
        assert_eq!(test_5, TestStatusCode::CORSError);
        assert_eq!(test_6, TestStatusCode::MethodNotAllowed);

        Ok(())
    }
}
