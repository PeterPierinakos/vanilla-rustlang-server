use crate::configuration::*;
use std::fs;

pub fn response_success(file: String) -> String {
    format!(
        "HTTP/2 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        file.len(),
        file
    )
}

pub fn response_400() -> String {
    let page_400 = fs::read_to_string(format!("{}/400.html", ABSOLUTE_STATIC_CONTENT_PATH)).expect(
        format!(
            "400 HTML page doesn't exist ('{}/400.html')",
            ABSOLUTE_STATIC_CONTENT_PATH
        )
        .as_str(),
    );

    format!(
        "HTTP/2 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}",
        page_400.len(),
        page_400
    )
}

pub fn response_404() -> String {
    let page_404 = fs::read_to_string(format!("{}/404.html", ABSOLUTE_STATIC_CONTENT_PATH)).expect(
        format!(
            "404 HTML page doesn't exist ('{}/400.html')",
            ABSOLUTE_STATIC_CONTENT_PATH
        )
        .as_str(),
    );

    format!(
        "HTTP/2 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
        page_404.len(),
        page_404,
    )
}
