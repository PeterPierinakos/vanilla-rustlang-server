use super::factory::ResponseFactory;
use crate::core::configuration::Configuration;
use crate::http::HttpProtocolVersion;
use crate::status::StatusCode;
use std::collections::HashMap;
use std::io;

/// Used for building an HTTP response.
///
/// # Examples
///
/// Example HTTP response:
///
/// HTTP/1.1 OK
pub struct ResponseBuilder<'a> {
    pub protocol: Option<&'a str>,
    pub status_code: Option<StatusCode>,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
    pub config: Configuration<'a>,
}

impl<'a> ResponseBuilder<'a> {
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

    pub fn body(&mut self, body: String) {
        self.body = Some(body);
    }

    pub fn add_header(&mut self, key: String, val: String) {
        self.headers.insert(key, val);
    }
}

impl ResponseFactory for ResponseBuilder<'_> {
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
