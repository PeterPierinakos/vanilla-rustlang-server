use std::fs;
use crate::configuration::*;

pub fn response_success(file: String) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        file.len(),
        file
    )
}

pub fn response_400() -> String {
    let page_400 =
        fs::read_to_string(format!("{}/html/400.html", ABSOLUTE_STATIC_CONTENT_PATH)).expect(
            format!(
                "400 HTML page doesn't exist ('{}/html/400.html')",
                ABSOLUTE_STATIC_CONTENT_PATH
            )
            .as_str(),
        );

    format!(
        "HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}",
        page_400.len(),
        page_400 
    )
}

pub fn response_404() -> String {
    let page_404 =
        fs::read_to_string(format!("{}/html/404.html", ABSOLUTE_STATIC_CONTENT_PATH)).expect(
            format!(
                "404 HTML page doesn't exist ('{}/html/400.html')",
                ABSOLUTE_STATIC_CONTENT_PATH
            )
            .as_str(),
        );

    format!(
        "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
        page_404.len(),
        page_404,
    )
}
