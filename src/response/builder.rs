use std::collections::HashMap;

use crate::configuration::{ALLOW_IFRAMES, HTTP_PROTOCOL_VERSION, SECURITY_HEADERS};
use crate::headers::Header;
use crate::http::HttpProtocolVersion;
use crate::status::StatusCode;

pub struct ResponseBuilder<'a> {
    protocol: Option<&'a str>,
    status_code: Option<StatusCode>,
    body: Option<&'a str>,
    headers: Header,
}

impl<'a> ResponseBuilder<'a> {
    pub fn new() -> Self {
        Self {
            protocol: None,
            status_code: None,
            body: None,
            headers: HashMap::new(),
        }
    }

    pub fn detect_protocol(&mut self) {
        if HTTP_PROTOCOL_VERSION == HttpProtocolVersion::OneDotOne {
            self.protocol = Some("HTTP/1.1");
        } else {
            self.protocol = Some("HTTP/2");
        }
    }

    /* Headers to prevent common attacks */
    /*

    List of security headers included:
    * X-Content-Type-Options
    * X-Frame-Options
    * Cross-Origin-Resource-Policy

    */
    pub fn apply_security_headers(&mut self) {
        if !SECURITY_HEADERS {
            return;
        }

        /* Prevent malicious HTML */
        self.add_header("X-Content-Type-Options".to_string(), "nosniff".to_string());

        /* Prevent clickjacking */
        if !ALLOW_IFRAMES {
            self.add_header("X-Frame-Options".to_string(), "DENY".to_string());
        }

        /* Prevent embedding resources from another origin */
        self.add_header(
            "Cross-Origin-Resource-Policy".to_string(),
            "same-origin".to_string(),
        );
    }

    pub fn status_code(&mut self, status: StatusCode) {
        self.status_code = Some(status);
    }

    pub fn body(&mut self, body: &'a str) {
        self.body = Some(body);
    }

    pub fn add_header(&mut self, key: String, val: String) {
        self.headers.insert(key, val);
    }

    pub fn get_headers(&self) -> Header {
        self.headers.clone()
    }

    pub fn construct(&self) -> String {
        let mut response = String::new();

        let str_status = self.status_code.unwrap();

        response.push_str(self.protocol.unwrap());
        response.push(' ');
        response.push_str(str_status.to_string().as_str());
        response.push(' ');

        response.push_str(match self.status_code.unwrap() {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            _ => panic!("Invalid status code provided"),
        });

        for (key, val) in &self.headers {
            response.push_str("\r\n");
            response.push_str(format!("{}:{}", key, val).as_str());
        }

        response.push_str("\r\n\r\n");
        response.push_str(self.body.unwrap());

        response
    }
}
