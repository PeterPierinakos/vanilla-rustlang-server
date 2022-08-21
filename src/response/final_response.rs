use super::builder::ResponseBuilder;
use std::fs;
use super::types::ResponseType;
use super::factory::ResponseFactory;
use super::utils::apply_extra_headers;
use super::htmlbuilder::HTMLBuilder;
use crate::core::configuration::Configuration;
use crate::error::ServerError;
use crate::file::*;
use crate::headers::Header;
use crate::status::StatusCode;
use std::io::{self, Error, ErrorKind, Read};
use std::time::{SystemTime, UNIX_EPOCH};

/// The "finalizer" struct for responses. Takes all the response data and turns them into a valid HTTP
/// response string using the `FinalResponse::build` method.
pub struct FinalResponse<'a> {
    pub status_code: StatusCode,
    pub req_headers: Option<Header>,
    pub config: &'a Configuration<'a>,
    // The field is wrapped inside Option because when the serve_request function initially calls
    // the builder it doesn't know about its required fields yet.
    pub response_type: Option<ResponseType>,
}

impl<'a> FinalResponse<'a> {
    /// Special default function reserved to be used by the serve_request core server method.
    pub fn special_default_builder(
        config: &'a Configuration,
    ) -> Self {
        Self {
            status_code: 200,
            req_headers: None,
            response_type: None,
            config,
        }
    }

    pub fn status_code(self, status_code: StatusCode) -> Self {
        Self {
            status_code,
            ..self
        }
    }

    pub fn req_headers(self, req_headers: Header) -> Self {
        Self {
            req_headers: Some(req_headers),
            ..self
        }
    }

    pub fn response_type(self, response_type: ResponseType) -> Self {
        Self {
            response_type: Some(response_type),
            ..self
        }
    }
}

impl ResponseFactory for FinalResponse<'_> {
    type ResponseContent = String;
    type ResponseError = ServerError;

    fn build(self) -> Result<Self::ResponseContent, Self::ResponseError> {
        let mut res = ResponseBuilder::new(self.config.clone());

        let response_type = match self.response_type {
            Some(response_type) => response_type,
            None => return Err(ServerError::from(io::Error::new(io::ErrorKind::InvalidInput, "ResponseFactory::build method called before passing response_type field to the response struct."))),
        };

        let req_headers = match self.req_headers {
            Some(req_headers) => req_headers,
            None => return Err(ServerError::from(io::Error::new(io::ErrorKind::InvalidInput, "ResponseFactory::build method called before passing req_headers field to the response struct."))),
        };
        
        res.detect_protocol();

        match response_type {
            ResponseType::File(mut res_data) => {
                let mut file_buf = String::new();

                if let Err(_) = res_data.file.read_to_string(&mut file_buf) {
                    return Err(ServerError::IOError(Error::new(
                        ErrorKind::InvalidData,
                        "File is not valid UTF-8 data.",
                    )));
                }

                if self.config.append_extra_headers {
                    apply_extra_headers(&mut res, &self.config.extra_headers);
                }

                if self.config.use_time_header {
                    let curr_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs()
                        .to_string();
                    res.add_header("Time".to_string(), curr_time);
                }

                // Apply CORS headers
                match req_headers.get("Access-Control-Allow-Origin") {
                    Some(val) => res.add_header("Access-Control-Allow-Origin".into(), val.into()),
                    None => res.add_header(
                        "Access-Control-Allow-Origin".to_string(),
                        "null".to_string(),
                    ),
                };

                // Apply necessary headers and security headers
                res.add_header(
                    "Content-Type".into(),
                    find_mime_type(res_data.file_ext.as_str()).to_string(),
                );
                res.add_header("Content-Length".into(), file_buf.len().to_string());

                if self.config.use_security_headers {
                    res.apply_security_headers();
                }

                res.body(file_buf);
                res.status_code(self.status_code);
            }
            ResponseType::Dir(res_data) => {
                if self.config.append_extra_headers {
                    apply_extra_headers(&mut res, &self.config.extra_headers);
                }

                // Apply CORS headers
                match req_headers.get("Access-Control-Allow-Origin") {
                    Some(val) => res.add_header("Access-Control-Allow-Origin".into(), val.into()),
                    None => res.add_header(
                        "Access-Control-Allow-Origin".to_string(),
                        "null".to_string(),
                    ),
                };

                let mut html = HTMLBuilder::new();

                html.add_to_body("Directory contents:");

                html.add_to_body("<ul>");

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
                    let filename = match filename.to_str() {
                        Some(str) => str,
                        None => {
                            return Err(ServerError::IOError(io::Error::new(
                                ErrorKind::Other,
                                "Failed parsing requested file name from OsString to str.",
                            )))
                        }
                    };

                    let decorated_filename = format!("{filename}");

                    dirs.push(decorated_filename);
                }

                for dir in &dirs {
                    html.add_to_body("<li>");
                    html.add_to_body(dir.as_str());
                    html.add_to_body("</li>")
                }

                html.add_to_body("</ul>");

                // Apply necessary headers and security headers
                res.add_header("Content-Type".into(), "text/html".into());
                res.add_header("Content-Length".into(), html.build().len().to_string());

                if self.config.use_security_headers {
                    res.apply_security_headers();
                }

                res.status_code(200);

                let doc = html.build();

                res.body(doc);
            },
            ResponseType::Fallback => {
                let fallback_file = match fs::read_to_string([self.config.absolute_static_content_path, "/", self.status_code.to_string().as_str(), ".html"].concat()) {
                    Ok(file) => file,
                    Err(_) => return Err(ServerError::from(io::Error::new(io::ErrorKind::NotFound, format!("Fallback file for {} status code page doesn't exist. Have you run the setup.sh script?", self.status_code))))
                };

                res.add_header("Content-Type".into(), "text/html".into());
                res.add_header("Content-Length".into(), fallback_file.len().to_string());
                res.body(fallback_file);
                res.status_code(self.status_code);
            }
        }

        Ok(res.build()?)
    }
}