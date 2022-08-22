use crate::headers::find_buf_headers;
use crate::response::ErrorResponse;
use crate::status::StatusCode;
use std::collections::HashMap;
use std::io::Read;

/* Verifies that the socket has valid request data, otherwise return the appropriate status code for the error. */
pub fn read_stream<'a>(
    mut stream: impl Read,
) -> Result<(HashMap<String, String>, Vec<u8>), StatusCode> {
    let mut buf = vec![0; 1024];

    let mut remaining_buf = buf.as_mut_slice();
    loop {
        match stream.read(remaining_buf) {
            // End the stream when there is no data left- this usually doesn't happen
            Ok(0) => break,
            Ok(count) => {
                // Check if headers have finished
                let at_end = remaining_buf[..count]
                    .iter()
                    .rev()
                    .scan(false, |last_was_nl, b| {
                        // Don't get confused by CRLF
                        if *b == b'\r' {
                            return Some(false);
                        }
                        // Check for the exit condition
                        if *last_was_nl && *b == b'\n' {
                            return Some(true);
                        }
                        // No match, continue
                        *last_was_nl = *b == b'\n';
                        Some(false)
                    })
                    .any(std::convert::identity);
                if at_end {
                    break;
                }
                // Grow to read more data
                remaining_buf = &mut remaining_buf[count..];
                if remaining_buf.is_empty() {
                    buf.extend(std::iter::repeat(0).take(1024));
                    let new_start = buf.len() - 1024;
                    remaining_buf = &mut buf[new_start..];
                }
            }
            Err(_) => return Err(400),
        }
    }

    Ok((find_buf_headers(&buf)?, buf))
}

pub fn parse_utf8(headers: &HashMap<String, String>, buf: &[u8]) -> Result<String, ErrorResponse> {
    let parsed_utf8 = std::str::from_utf8(buf);

    match parsed_utf8 {
        Ok(string) => Ok(string.to_string()),
        Err(_) => Err((headers.clone(), 400)),
    }
}
