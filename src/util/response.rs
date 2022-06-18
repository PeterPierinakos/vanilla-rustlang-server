use super::headers::Header;
use crate::configuration::*;
use crate::enums::server::{ServerError, StatusCode};
use crate::structs::responsebuilder::ResponseBuilder;
use std::fs::{self, File};
use std::io::Read;

pub type ErrorResponse<'a> = (Header, ServerError);
pub type OkResponse<'a> = (Header, StatusCode, Option<File>);

/* Headers, Status Code, Response File */
pub type ServerResponse<'a> = Result<OkResponse<'a>, ErrorResponse<'a>>;

pub fn handle_response(response: ServerResponse) -> String {
    match response {
        Ok((headers, StatusCode::OK, file)) => create_response(headers, file, 200),
        Ok((headers, StatusCode::BadRequest, _)) => create_response(headers, None, 400),
        Ok((headers, StatusCode::MethodNotAllowed, _)) => create_response(headers, None, 405),
        Ok((headers, StatusCode::NotFound, _)) => create_response(headers, None, 404),
        Ok((headers, StatusCode::InternalServerError, _)) => create_response(headers, None, 500),
        Err((headers, ServerError::ParseIntegerError(_))) => create_response(headers, None, 400),
        Err((headers, ServerError::ParseUtf8Error(_))) => create_response(headers, None, 400),
        Err((headers, ServerError::StreamError)) => create_response(headers, None, 400),
        Err((headers, ServerError::BufferHeaderError)) => create_response(headers, None, 400),
    }
}

pub fn create_response(req_headers: Header, file: Option<File>, status_code: u16) -> String {
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
    // Apply necessary headers and security headers
    response.add_header("Content-Type".into(), "text/html".into());
    response.add_header("Content-Length".into(), file_buf.len().to_string());
    response.apply_security_headers();

    response.detect_protocol();
    response.body(file_buf.as_str());
    response.status_code(StatusCode::OK);
    response.add_header("Content-Type".into(), "text/html".into());
    response.add_header("Content-Length".into(), file_buf.len().to_string());
    response.construct()
}

fn find_file(filename: &str) -> File {
    let url = [ABSOLUTE_STATIC_CONTENT_PATH, "/", filename].concat();

    let file =
        fs::File::open(&url).expect(format!("{filename} file doesn't exist ('{}')", &url).as_str());

    file
}
