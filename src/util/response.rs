use crate::configuration::*;
use std::fs;

pub fn response_success(file: String) -> String {
    let ln = file.len();
    format!("HTTP/2 200 OK\r\nContent-Length: {ln}\r\n\r\n{file}",)
}

pub fn response_400() -> String {
    let page_400 = fs::read_to_string(format!("{ABSOLUTE_STATIC_CONTENT_PATH}/400.html")).expect(
        format!("400 HTML page doesn't exist ('{ABSOLUTE_STATIC_CONTENT_PATH}/400.html')",)
            .as_str(),
    );

    let ln = page_400.len();

    format!("HTTP/2 400 Bad Request\r\nContent-Length: {ln}\r\n\r\n{page_400}",)
}

pub fn response_404() -> String {
    let page_404 = fs::read_to_string(format!("{ABSOLUTE_STATIC_CONTENT_PATH}/404.html")).expect(
        format!("404 HTML page doesn't exist ('{ABSOLUTE_STATIC_CONTENT_PATH}/400.html')").as_str(),
    );

    let ln = page_404.len();

    format!("HTTP/2 404 Not Found\r\nContent-Length: {ln}\r\n\r\n{page_404}")
}
