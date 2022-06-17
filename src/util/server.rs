use crate::configuration::*;
use crate::enums::http::HttpRequestMethod;
use crate::enums::server::StatusCode;
use crate::structs::cors::Cors;
use crate::structs::uri::URI;
use crate::util::license::print_license_info;
use crate::util::response::ServerResponse;
use crate::util::response::*;
use crate::util::socket::parse_utf8;
use crate::util::socket::read_stream;
use crate::util::time::generate_unixtime;
use std::cell::RefCell;
use std::collections::HashSet;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::{fs, fs::File, fs::OpenOptions};

use super::headers::Header;

fn multithread_handle_connection<'a>(cors: Cors, stream: &'a mut TcpStream) -> ServerResponse<'a> {
    let (req_headers, buf) = read_stream(stream)?;

    let req_headers_ref = RefCell::from(req_headers);

    let buf_utf8 = parse_utf8(&req_headers_ref, &buf)?;

    let origin = match req_headers_ref.borrow().get("Origin") {
        Some(header) => header.to_string(),
        None => "null".to_string(),
    };

    req_headers_ref
        .borrow_mut()
        .insert(
            "Access-Control-Allow-Origin".to_string(),
            if ALLOW_ALL_ORIGINS {
                "*".to_string()
            } else if cors.origin_is_allowed(&origin) {
                origin.to_string()
            } else {
                "null".to_string()
            },
        )
        .unwrap();

    main_logic(req_headers_ref, buf_utf8, cors, stream)
}

fn singlethread_handle_connection<'a>(
    cors: Cors,
    logfile: &'a Option<File>,
    stream: &'a mut TcpStream,
) -> ServerResponse<'a> {
    let (req_headers, buf) = read_stream(stream)?;

    let req_headers_ref = RefCell::from(req_headers.clone());

    let origin = match req_headers_ref.borrow().get("Origin") {
        Some(header) => header.to_string(),
        None => "null".to_string(),
    };

    req_headers_ref.borrow_mut().insert(
        "Access-Control-Allow-Origin".to_string(),
        if ALLOW_ALL_ORIGINS {
            "*".to_string()
        } else if cors.origin_is_allowed(&origin) {
            origin.to_string()
        } else {
            "null".to_string()
        },
    );

    let buf_utf8 = parse_utf8(&req_headers_ref, &buf)?;

    main_logic(req_headers_ref, buf_utf8, cors, stream)
}

fn main_logic<'a>(
    req_headers_ref: RefCell<Header>,
    buf_utf8: String,
    cors: Cors,
    stream: &'a TcpStream,
) -> ServerResponse<'a> {
    if !cors.method_is_allowed(&buf_utf8) {
        return Ok((req_headers_ref, StatusCode::MethodNotAllowed, None));
    }

    let mut uri = URI::new();

    uri.find(&buf_utf8);

    if uri.get() == &None {
        return Ok((req_headers_ref, StatusCode::BadRequest, None));
    };

    let requested_content = fs::read_to_string(format!(
        "{ABSOLUTE_STATIC_CONTENT_PATH}/{}",
        uri.get().clone().unwrap()
    ));
    let response = match requested_content {
        Ok(file) => file,
        Err(_err) => {
            return Ok((req_headers_ref, StatusCode::NotFound, None));
        }
    };

    Ok((req_headers_ref.clone(), StatusCode::OK, Some(response)))
}

pub fn start_server(unixtime: u64) {
    print_license_info();

    let listener = TcpListener::bind(format!("{ADDR}:{PORT}")).unwrap();

    let cors = Cors::new(HashSet::from([HttpRequestMethod::GET]));

    let logfile = if SAVE_LOGS {
        Some(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("{ABSOLUTE_LOGS_PATH}/{unixtime}.txt"))
                .expect("Unable to open logs"),
        )
    } else {
        None
    };

    if !SECURITY_HEADERS {
        println!("Production note: security headers are currently turned off, keep it enabled in production!")
    }

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let cors_ref = cors.clone();

        if MULTITHREADING {
            thread::spawn(move || {
                let handled = multithread_handle_connection(cors_ref, &mut stream);

                let response = handle_response(handled);

                stream.write(&response.as_bytes()).unwrap();
                stream.flush().unwrap();
            });
        } else {
            let handled = singlethread_handle_connection(cors_ref, &logfile, &mut stream);

            let response = handle_response(handled);

            stream.write(&response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
