use super::builder::ResponseBuilder;
use super::htmlbuilder::HTMLBuilder;
use crate::core::configuration::Configuration;
use crate::error::ServerError;
use crate::file::*;
use crate::headers::Header;
use crate::status::StatusCode;
use std::fs::{self, File};
use std::io::{self, Error, ErrorKind, Read};

pub type ErrorResponse = (Header, StatusCode);
pub type OkResponse = (Header, Option<String>, Option<File>);

/* Headers, Status Code, Response File */
pub type ServerResponse<'a> = Result<OkResponse, ErrorResponse>;

/// Returns a full HTTP response with the requested file.
///
/// # Panics
///
/// Panics if the provided status code is invalid.
pub fn create_file_response(
    req_headers: Header,
    file_ext: Option<String>,
    file: Option<File>,
    status_code: u16,
    config: &Configuration,
) -> Result<String, ServerError> {
    let mut file = match file {
        Some(content) => content,
        None => match status_code {
            400 => find_file("400.html"),
            404 => find_file("404.html"),
            500 => find_file("500.html"),
            405 => find_file("405.html"),
            _ => panic!("Invalid status code passed to create_response"),
        },
    };

    let mut file_buf = String::new();

    if let Err(_) = file.read_to_string(&mut file_buf) {
        return Err(ServerError::IOError(Error::new(
            ErrorKind::InvalidData,
            "File is not valid UTF-8 data.",
        )));
    }

    let mut response = ResponseBuilder::new();

    if config.append_extra_headers {
        apply_extra_headers(&mut response, &config.extra_headers);
    }

    // Apply CORS headers
    match req_headers.get("Access-Control-Allow-Origin") {
        Some(val) => response.add_header("Access-Control-Allow-Origin".into(), val.into()),
        None => response.add_header(
            "Access-Control-Allow-Origin".to_string(),
            "null".to_string(),
        ),
    };

    let file_ext = match file_ext {
        Some(ext) => ext,
        None => "html".to_string(),
    };

    // Apply necessary headers and security headers
    response.add_header(
        "Content-Type".into(),
        find_mime_type(file_ext.as_str()).to_string(),
    );
    response.add_header("Content-Length".into(), file_buf.len().to_string());
    response.apply_security_headers();

    response.detect_protocol();
    response.body(file_buf.as_str());
    response.status_code(status_code);

    Ok(response.construct())
}

pub fn create_dir_response(
    req_headers: Header,
    path_iterator: fs::ReadDir,
    config: &Configuration,
) -> Result<String, ServerError> {
    let mut response = ResponseBuilder::new();

    if config.append_extra_headers {
        apply_extra_headers(&mut response, &config.extra_headers);
    }

    // Apply CORS headers
    match req_headers.get("Access-Control-Allow-Origin") {
        Some(val) => response.add_header("Access-Control-Allow-Origin".into(), val.into()),
        None => response.add_header(
            "Access-Control-Allow-Origin".to_string(),
            "null".to_string(),
        ),
    };

    let mut html = HTMLBuilder::new();

    html.add_to_body("Directory contents:");

    html.add_to_body("<ul>");

    let mut dirs: Vec<String> = vec![];

    for item in path_iterator {
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
    response.detect_protocol();
    response.add_header("Content-Type".into(), "text/html".into());
    response.add_header("Content-Length".into(), html.construct().len().to_string());
    response.apply_security_headers();
    response.status_code(200);

    let doc = html.construct();

    response.body(doc.as_str());

    Ok(response.construct())
}

/// Used to apply the extra headers specified in the configuration file.
///
/// Borrows an instance of `ResponseBuilder` in order to apply them.
pub fn apply_extra_headers(response: &mut ResponseBuilder, extra_headers: &Vec<[&str; 2]>) {
    for header in extra_headers.iter() {
        response.add_header(header[0].into(), header[1].into())
    }
}
