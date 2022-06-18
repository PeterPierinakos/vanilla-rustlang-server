use super::headers::{standard_headers, Header};
use crate::configuration::*;
use crate::enums::server::{ServerError, StatusCode};
use crate::structs::responsebuilder::ResponseBuilder;
use std::cell::RefCell;
use std::fs;

pub type ErrorResponse<'a> = (RefCell<Header>, ServerError);
pub type OkResponse<'a> = (RefCell<Header>, StatusCode, Option<String>);

/* Headers, Status Code, Response File */
pub type ServerResponse<'a> = Result<OkResponse<'a>, ErrorResponse<'a>>;

pub fn handle_response(response: ServerResponse) -> String {
    match response {
        Ok((headers, StatusCode::OK, file)) => response_success(headers, file.unwrap()),
        Ok((headers, StatusCode::BadRequest, _)) => response_400(headers),
        Ok((headers, StatusCode::MethodNotAllowed, _)) => response_405(headers),
        Ok((headers, StatusCode::NotFound, _)) => response_404(headers),
        Ok((headers, StatusCode::InternalServerError, _)) => response_500(headers),
        Err((headers, ServerError::ParseIntegerError(_))) => response_400(headers),
        Err((headers, ServerError::ParseUtf8Error(_))) => response_400(headers),
        Err((headers, ServerError::StreamError)) => response_500(headers),
        Err((headers, ServerError::BufferHeaderError)) => response_500(headers),
    }
}

pub fn implement_cors_header(req_headers: &Header, res_headers: &mut Header) {
    match req_headers.get("Access-Control-Allow-Origin") {
        Some(val) => res_headers.insert("Access-Control-Allow-Origin".to_string(), val.to_string()),
        None => res_headers.insert(
            "Access-Control-Allow-Origin".to_string(),
            "null".to_string(),
        ),
    };
}

pub fn response_success(req_headers: RefCell<Header>, file: String) -> String {
    let mut res_headers = standard_headers(&file);

    implement_cors_header(&req_headers.borrow(), &mut res_headers);

    let mut response = ResponseBuilder::new();

    response.apply_security_headers();
    response.detect_protocol();
    response.body(file.as_str());
    response.status_code(StatusCode::OK);

    for (key, val) in res_headers {
        response.add_header(key, val);
    }

    response.construct()
}

pub fn response_400(req_headers: RefCell<Header>) -> String {
    let page_400 = find_file("400.html");

    let mut res_headers = standard_headers(&page_400);

    implement_cors_header(&req_headers.borrow(), &mut res_headers);

    let mut response = ResponseBuilder::new();

    let file = find_file("400.html");

    response.apply_security_headers();
    response.detect_protocol();
    response.body(file.as_str());
    response.status_code(StatusCode::OK);

    for (key, val) in res_headers {
        response.add_header(key, val);
    }

    response.construct()
}

pub fn response_404(req_headers: RefCell<Header>) -> String {
    let page_404 = find_file("404.html");

    let mut res_headers = standard_headers(&page_404);

    implement_cors_header(&req_headers.borrow(), &mut res_headers);

    let mut response = ResponseBuilder::new();

    let file = find_file("404.html");

    response.apply_security_headers();
    response.detect_protocol();
    response.body(file.as_str());
    response.status_code(StatusCode::OK);

    for (key, val) in res_headers {
        response.add_header(key, val);
    }

    response.construct()
}

pub fn response_405(req_headers: RefCell<Header>) -> String {
    let page_405 = find_file("405.html");

    let mut res_headers = standard_headers(&page_405);

    implement_cors_header(&req_headers.borrow(), &mut res_headers);

    let mut response = ResponseBuilder::new();

    let file = find_file("405.html");

    response.apply_security_headers();
    response.detect_protocol();
    response.body(file.as_str());
    response.status_code(StatusCode::OK);

    for (key, val) in res_headers {
        response.add_header(key, val);
    }

    response.construct()
}

pub fn response_500(req_headers: RefCell<Header>) -> String {
    let page_500 = find_file("500.html");

    let mut res_headers = standard_headers(&page_500);

    implement_cors_header(&req_headers.borrow(), &mut res_headers);

    let mut response = ResponseBuilder::new();

    let file = find_file("500.html");

    response.apply_security_headers();
    response.detect_protocol();
    response.body(file.as_str());
    response.status_code(StatusCode::OK);

    for (key, val) in res_headers {
        response.add_header(key, val);
    }

    response.construct()
}

fn find_file(filename: &str) -> String {
    let url = [ABSOLUTE_STATIC_CONTENT_PATH, "/", filename].concat();

    let file = fs::read_to_string(&url)
        .expect(format!("{filename} file doesn't exist ('{}')", &url).as_str());

    file
}
