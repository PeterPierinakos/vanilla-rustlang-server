use super::headers::{standard_headers, Header};
use crate::configuration::HTTP_PROTOCOL_VERSION;
use crate::configuration::*;
use crate::enums::http::HttpProtocolVersion;
use crate::enums::server::{ServerError, StatusCode};
use crate::structs::cors::Cors;
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
        Ok((_, StatusCode::InternalServerError, _)) => response_500(),
        Err((headers, ServerError::ParseIntegerError(_))) => response_400(headers),
        Err((headers, ServerError::ParseUtf8Error(_))) => response_400(headers),
        Err((_, ServerError::StreamError)) => response_500(),
        Err((_, ServerError::BufferHeaderError)) => response_500(),
        Err((headers, ServerError::MissingHeaderError)) => response_400(headers),
        Err((headers, ServerError::CorsError)) => response_400(headers),
    }
}

pub fn find_protocol() -> &'static str {
    if HTTP_PROTOCOL_VERSION == HttpProtocolVersion::OneDotOne {
        return "HTTP/1.1";
    }
    "HTTP/2"
}

pub fn response_builder(
    file: String,
    protocol: &str,
    status_code: StatusCode,
    headers: Header,
) -> String {
    let mut response = String::new();

    response.push_str(format!("{protocol} ").as_str());

    response.push_str(match status_code {
        StatusCode::OK => "200 OK ",
        StatusCode::BadRequest => "400 Bad Request ",
        StatusCode::NotFound => "404 Not Found ",
        StatusCode::MethodNotAllowed => "405 Method Not Allowed ",
        StatusCode::InternalServerError => "500 Internal Server Error ",
    });

    for (key, val) in headers.iter() {
        response.push_str("\r\n");
        response.push_str(format!("{key}: {val}").as_str());
    }

    response.push_str("\r\n\r\n");
    response.push_str(format!("{file}").as_str());

    response
}

pub fn response_success(req_headers: RefCell<Header>, file: String) -> String {
    let mut res_headers = standard_headers(&file);

    match req_headers.borrow().get("Access-Control-Allow-Origin") {
        Some(val) => res_headers.insert("Access-Control-Allow-Origin".to_string(), val.to_string()),
        None => res_headers.insert(
            "Access-Control-Allow-Origin".to_string(),
            "null".to_string(),
        ),
    };

    response_builder(file, find_protocol(), StatusCode::OK, res_headers)
}

pub fn response_400(req_headers: RefCell<Header>) -> String {
    let page_400 = fs::read_to_string(format!("{ABSOLUTE_STATIC_CONTENT_PATH}/400.html")).expect(
        format!("400 HTML page doesn't exist ('{ABSOLUTE_STATIC_CONTENT_PATH}/400.html')",)
            .as_str(),
    );

    let res_headers = standard_headers(&page_400);

    response_builder(
        page_400,
        find_protocol(),
        StatusCode::BadRequest,
        res_headers,
    )
}

pub fn response_404(req_headers: RefCell<Header>) -> String {
    let page_404 = fs::read_to_string(format!("{ABSOLUTE_STATIC_CONTENT_PATH}/404.html")).expect(
        format!("404 HTML page doesn't exist ('{ABSOLUTE_STATIC_CONTENT_PATH}/404.html')").as_str(),
    );

    let res_headers = standard_headers(&page_404);

    response_builder(
        page_404,
        find_protocol(),
        StatusCode::BadRequest,
        res_headers,
    )
}

pub fn response_405(req_headers: RefCell<Header>) -> String {
    let page_405 = fs::read_to_string(format!("{ABSOLUTE_STATIC_CONTENT_PATH}/405.html")).expect(
        format!("405 HTML page doesn't exist ('{ABSOLUTE_STATIC_CONTENT_PATH}/405.html')").as_str(),
    );

    let res_headers = standard_headers(&page_405);

    response_builder(
        page_405,
        find_protocol(),
        StatusCode::BadRequest,
        res_headers,
    )
}

pub fn response_500() -> String {
    let page_500 = fs::read_to_string(format!("{ABSOLUTE_STATIC_CONTENT_PATH}/500.html")).expect(
        format!("500 HTML page doesn't exist ('{ABSOLUTE_STATIC_CONTENT_PATH}/500.html')").as_str(),
    );

    let res_headers = standard_headers(&page_500);

    response_builder(
        page_500,
        find_protocol(),
        StatusCode::BadRequest,
        res_headers,
    )
}
