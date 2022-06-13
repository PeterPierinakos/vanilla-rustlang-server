use crate::configuration::HTTP_PROTOCOL_VERSION;
use crate::configuration::*;
use crate::enums::methods::HttpProtocolVersion;
use std::fs;

pub fn find_protocol() -> &'static str {
    if HTTP_PROTOCOL_VERSION == HttpProtocolVersion::OneDotOne {
        return "HTTP/1.1";
    }
    "HTTP/2"
}

pub fn response_success(file: String) -> String {
    let ln = file.len();

    let protocol = find_protocol();

    format!("{protocol} 200 OK\r\nContent-Type: text/html\r\nContent-Length: {ln}\r\n\r\n{file}",)
}

pub fn response_400() -> String {
    let page_400 = fs::read_to_string(format!("{ABSOLUTE_STATIC_CONTENT_PATH}/400.html")).expect(
        format!("400 HTML page doesn't exist ('{ABSOLUTE_STATIC_CONTENT_PATH}/400.html')",)
            .as_str(),
    );

    let protocol = find_protocol();

    let ln = page_400.len();

    format!("{protocol} 400 Bad Request\r\nContent-Type: text/html\r\nContent-Length: {ln}\r\n\r\n{page_400}",)
}

pub fn response_404() -> String {
    let page_404 = fs::read_to_string(format!("{ABSOLUTE_STATIC_CONTENT_PATH}/404.html")).expect(
        format!("404 HTML page doesn't exist ('{ABSOLUTE_STATIC_CONTENT_PATH}/400.html')").as_str(),
    );

    let protocol = find_protocol();

    let ln = page_404.len();

    format!(
        "{protocol} 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {ln}\r\n\r\n{page_404}"
    )
}
