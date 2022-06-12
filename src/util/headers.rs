use std::str;
use std::collections::HashMap;

pub fn find_buf_headers(buf: &[u8; 1024]) -> Vec<HashMap<String, String>> {
        let buffer_c = str::from_utf8(buf).unwrap(); 

        let mut headers: Vec<HashMap<String, String>> = Vec::new();

        let mut curr_header_name = String::new();
        let mut curr_header_value = String::new();

        let mut found_colon = false;

        for c in buffer_c.chars() {
            if c == ':' {
                found_colon = true;
            }
            else if c == '\r' || c == '\n' {
                if !curr_header_name.is_empty() && !curr_header_value.is_empty() {
                    headers.push(HashMap::from([(curr_header_name.clone(), curr_header_value.clone())]));
                    
                }
                curr_header_name = String::new();
                curr_header_value = String::new();
                found_colon = false;
            }
            else {
                if !found_colon {
                    curr_header_name.push(c);
                }
                else if found_colon && c != ' ' {
                    curr_header_value.push(c);
                }
            }
        };

        headers

}