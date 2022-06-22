use std::collections::HashMap;
use std::str;

use super::response::ErrorResponse;

pub type Header = HashMap<String, String>;

pub fn find_buf_headers(buf: &[u8; 1024]) -> Result<HashMap<String, String>, ErrorResponse> {
    let buffer_c = str::from_utf8(buf).unwrap();

    let mut headers: HashMap<String, String> = HashMap::new();

    let mut curr_header_name = String::new();
    let mut curr_header_value = String::new();

    let mut found_colon = false;

    for c in buffer_c.chars() {
        if c == ':' {
            found_colon = true;
        } else if c == '\r' || c == '\n' {
            if !curr_header_name.is_empty() && !curr_header_value.is_empty() {
                headers.insert(curr_header_name, curr_header_value);
            }
            curr_header_name = String::new();
            curr_header_value = String::new();
            found_colon = false;
        } else if !found_colon {
            curr_header_name.push(c);
        } else if found_colon && c != ' ' {
            curr_header_value.push(c);
        }
    }

    if headers.is_empty() {
        return Err((headers, 400));
    }

    Ok(headers)
}
