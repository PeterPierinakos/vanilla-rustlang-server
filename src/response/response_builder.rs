use super::types::ResponseType;
use super::utils::*;
use crate::core::configuration::Configuration;
use crate::error::ServerError;
use crate::file::*;
use crate::status::StatusCode;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};

/// The "finalizer" struct for responses. Takes all the response data and turns them into a valid HTTP
/// response string.
pub struct ResponseBuilder<'a> {
    pub status_code: StatusCode,
    pub req_headers: Option<HashMap<String, String>>,
    pub config: &'a Configuration<'a>,
    // The field is wrapped inside Option because when the serve_request function initially calls
    // the builder it doesn't know about its required fields yet.
    pub response_type: Option<ResponseType<'a>>,
}

impl<'a> ResponseBuilder<'a> {
    /// Special default function reserved to be used by the serve_request core server method.
    pub fn status_code(self, status_code: StatusCode) -> Self {
        Self {
            status_code,
            ..self
        }
    }

    pub fn req_headers(self, req_headers: HashMap<String, String>) -> Self {
        Self {
            req_headers: Some(req_headers),
            ..self
        }
    }

    pub fn response_type(self, response_type: ResponseType<'a>) -> Self {
        Self {
            response_type: Some(response_type),
            ..self
        }
    }

    pub fn build(self) -> Result<String, ServerError> {
        let response_type = match self.response_type {
            Some(response_type) => response_type,
            None => return Err(ServerError::from(io::Error::new(io::ErrorKind::InvalidInput, "Builder function was ran before giving a valid response_type."))),
        };

        let mut head: Vec<String> = vec![];
        let mut body: Vec<String> = vec![];
        let protocol: &str = self.config.http_protocol_version.into();
        let status_code;

        let mut headers = match self.req_headers {
            Some(req_headers) => req_headers,
            None => HashMap::new(),
        };
        
        if self.config.use_security_headers {
            /* Prevent malicious HTML */
            headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());

            /* Prevent clickjacking */
            if !self.config.allow_iframes {
                headers.insert("X-Frame-Options".to_string(), "DENY".to_string());
            }

            /* Prevent embedding resources from another origin */
            headers.insert(
                "Cross-Origin-Resource-Policy".to_string(),
                "same-origin".to_string(),
            );
        }

        if self.config.append_extra_headers {
            for (key, val) in &self.config.extra_headers {
                headers.insert(key.to_string(), val.to_string());
            }
        }

        // Apply CORS headers
        if headers.get("Access-Control-Allow-Origin").is_none() {
            headers.insert(
                "Access-Control-Allow-Origin".into(),
                "null".into(),
            );
        };

        match response_type {
            ResponseType::File(res_data) => {
                if self.config.use_time_header {
                    let curr_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs()
                        .to_string();
                    headers.insert("Time".to_string(), curr_time);
                }

                // Apply necessary headers and security headers
                headers.insert(
                    "Content-Type".into(),
                    find_mime_type(res_data.file_ext).to_string(),
                );
                headers.insert(
                    "Content-Length".into(),
                    res_data.file_content.len().to_string(),
                );

                body.push(res_data.file_content.into());
                status_code = self.status_code;
            }
            ResponseType::Dir(res_data) => {
                match self.config.format_directory_listing_as_json {
                    false => {
                        head.push("<meta charset=\"utf-8\">".into());

                        let mut dirs: Vec<String> = vec![];

                        for item in res_data.path_iterator {
                            let item = match item {
                                Ok(item) => item,
                                Err(_) => {
                                    return Err(ServerError::IOError(Error::new(
                                        ErrorKind::Other,
                                        "Failed reading directory item",
                                    )))
                                }
                            };

                            let filename = item.file_name();
                            let filename =
                                match filename.to_str() {
                                    Some(str) => str,
                                    None => return Err(ServerError::IOError(io::Error::new(
                                        ErrorKind::Other,
                                        "Failed parsing requested file name from OsString to str.",
                                    ))),
                                };

                            let decorated_filename = format!("{filename}");

                            dirs.push(decorated_filename);
                        }

                        if dirs.is_empty() {
                            body.push("<p>The requested directory is empty.</p>".into());
                            status_code = 404;
                        } else {
                            body.push("<p>Directory contents:</p>".into());
                            body.push("<ul>".into());

                            for dir in &dirs {
                                body.push("<li>".into());
                                body.push(dir.as_str().into());
                                body.push("</li>".into())
                            }

                            body.push("</ul>".to_string());

                            let doc_len = build_html(head.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), body.iter().map(|s| s.as_str()).collect::<Vec<&str>>()).len();

                            // Apply necessary headers and security headers
                            headers.insert("Content-Type".into(), "text/html".into());
                            headers.insert("Content-Length".into(), doc_len.to_string());

                            status_code = 200;
                        }
                    }
                    true => {
                        // Apply necessary headers and security headers
                        headers.insert("Content-Type".into(), "application/json".into());

                        let mut dirs: Vec<String> = vec![];

                        for item in res_data.path_iterator {
                            let item = match item {
                                Ok(item) => item,
                                Err(_) => {
                                    return Err(ServerError::IOError(Error::new(
                                        ErrorKind::Other,
                                        "Failed reading directory item",
                                    )))
                                }
                            };

                            let filename = item.file_name();
                            let filename =
                                match filename.to_str() {
                                    Some(str) => str,
                                    None => return Err(ServerError::IOError(io::Error::new(
                                        ErrorKind::Other,
                                        "Failed parsing requested file name from OsString to str.",
                                    ))),
                                };

                            dirs.push(filename.to_string());
                        }

                        if dirs.is_empty() {
                            let json =
                                HashMap::from([("response", "The requested directory is empty.")]);
                            let json = map_as_json_str(json);
                            status_code = 404;
                            headers.insert("Content-Length".into(), json.len().to_string());
                            body.push(json.to_string());
                        } else {
                            let json = vec_as_json_str(dirs);

                            headers.insert("Content-Length".into(), json.len().to_string());

                            status_code = 200;

                            body.push(json.to_string());
                        }
                    }
                }
            }
            ResponseType::Fallback => {
                let fallback_file = match fs::read_to_string([self.config.absolute_static_content_path, "/", self.status_code.to_string().as_str(), ".html"].concat()) {
                    Ok(file) => file,
                    Err(_) => return Err(ServerError::from(io::Error::new(io::ErrorKind::NotFound, format!("Fallback file for {} status code page doesn't exist. Have you run 'make migrate'?", self.status_code))))
                };

                headers.insert("Content-Type".into(), "text/html".into());
                headers.insert("Content-Length".into(), fallback_file.len().to_string());
                body.push(fallback_file);
                status_code = self.status_code;
            }
        }

        let doc = build_html(head.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), body.iter().map(|s| s.as_str()).collect::<Vec<&str>>());

        let mut res = String::new();

        res.push_str(protocol);
        res.push(' ');
        res.push_str(status_code.to_string().as_str());
        res.push(' ');

        res.push_str(match status_code {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            _ => return Err(ServerError::from(io::Error::new(io::ErrorKind::InvalidInput, "Invalid status code provided. This should not occur under any circumstance in production, if this has occurred please report it on GitHub."))),
        });

        for (key, val) in headers {
            res.push_str("\r\n");
            res.push_str(format!("{}:{}", key, val).as_str());
        }

        res.push_str("\r\n\r\n");
        res.push_str(&doc);

        Ok(res)
    }
}
