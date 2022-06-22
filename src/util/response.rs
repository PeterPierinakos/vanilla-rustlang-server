use super::headers::Header;
use super::status::StatusCode;
use crate::configuration::*;
use crate::structs::responsebuilder::ResponseBuilder;
use std::fs::{self, File};
use std::io::Read;

pub type ErrorResponse = (Header, StatusCode);
pub type OkResponse = (Header, Option<String>, Option<File>);

/* Headers, Status Code, Response File */
pub type ServerResponse<'a> = Result<OkResponse, ErrorResponse>;

pub fn handle_response(response: ServerResponse) -> String {
    match response {
        Ok((headers, file_ext, file)) => create_response(&headers, file_ext, file, 200),
        Err((headers, status)) => create_response(&headers, None, None, status),
    }
}

pub fn create_response(
    req_headers: &Header,
    file_ext: Option<String>,
    file: Option<File>,
    status_code: u16,
) -> String {
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

    file.read_to_string(&mut file_buf)
        .expect("Requested file is not valid UTF-8 data.");

    let mut response = ResponseBuilder::new();

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
    response.status_code(200);
    response.construct()
}

fn find_file(filename: &str) -> File {
    let url = [ABSOLUTE_STATIC_CONTENT_PATH, "/", filename].concat();

    fs::File::open(&url).unwrap_or_else(|_| panic!("{filename} file doesn't exist ('{}')", url))
}

fn find_mime_type(file_extension: &str) -> &str {
    match file_extension {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => "text/plain",
    }
}
