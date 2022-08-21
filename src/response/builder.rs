use crate::core::configuration::Configuration;
use std::io;
use super::factory::ResponseFactory;
use crate::headers::Header;
use crate::http::HttpProtocolVersion;
use crate::status::StatusCode;
use std::collections::HashMap;

/// Used for building an HTTP response.
///
/// # Examples
///
/// Example HTTP response:
///
/// HTTP/1.1 OK
pub struct ResponseBuilder<'a, S: AsRef<str>> {
    protocol: Option<&'a str>,
    status_code: Option<StatusCode>,
    body: Option<S>,
    headers: Header,
    config: Configuration<'a>,
}

impl<'a, S: AsRef<str>> ResponseBuilder<'a, S> {
    pub fn new(config: Configuration<'a>) -> Self {
        Self {
            protocol: None,
            status_code: None,
            body: None,
            headers: HashMap::new(),
            config: config,
        }
    }

    pub fn detect_protocol(&mut self) {
        if self.config.http_protocol_version == HttpProtocolVersion::OneDotOne {
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
        /* Prevent malicious HTML */
        self.add_header("X-Content-Type-Options".to_string(), "nosniff".to_string());

        /* Prevent clickjacking */
        if !self.config.allow_iframes {
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

    pub fn body(&mut self, body: S) {
        self.body = Some(body);
    }

    pub fn add_header(&mut self, key: String, val: String) {
        self.headers.insert(key, val);
    }

    pub fn get_headers(&self) -> Header {
        self.headers.clone()
    }
}

impl<S: AsRef<str>> ResponseFactory for ResponseBuilder<'_, S> {
    type ResponseContent = String;
    type ResponseError = io::Error;

    fn build(self) -> Result<Self::ResponseContent, Self::ResponseError> {
        let mut response = String::new();

        let status_code = match self.status_code {
            Some(status_code) => status_code,
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "Tried to call ResponseFactory::build method before assigning required field 'status_code'."))
        };
        let protocol = match self.protocol {
            Some(protocol) => protocol,
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "Tried to call ResponseFactory::build method before assigning required field 'protocol'."))
        };

        response.push_str(protocol);
        response.push(' ');
        response.push_str(status_code.to_string().as_str());
        response.push(' ');

        response.push_str(match status_code {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid status code provided. This should not occur under any circumstance in production, if this has occurred please report it on GitHub.")),
        });

        for (key, val) in &self.headers {
            response.push_str("\r\n");
            response.push_str(format!("{}:{}", key, val).as_str());
        }

        response.push_str("\r\n\r\n");
        response.push_str(self.body.unwrap().as_ref());

        Ok(response)
    }
}
