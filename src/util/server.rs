use crate::configuration::*;
use crate::enums::http::HttpRequestMethod;
use crate::enums::server::ServerError;
use crate::enums::server::StatusCode;
use crate::structs::cors::Cors;
use crate::structs::uri::URI;
use crate::util::license::print_license_info;
use crate::util::response::*;
use crate::util::socket::parse_utf8;
use crate::util::socket::read_stream;
use crate::util::time::generate_unixtime;
use std::collections::HashSet;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::{fs, fs::File, fs::OpenOptions};

fn multithread_handle_connection(
    stream: &mut TcpStream,
) -> Result<(StatusCode, Option<String>), ServerError> {
    let buf = read_stream(stream)?;

    let buf_utf8 = parse_utf8(&buf)?;

    let cors = Cors::new(HashSet::from([HttpRequestMethod::GET]));

    if !cors.method_is_allowed(&buf_utf8) {
        return Ok((StatusCode::MethodNotAllowed, None));
    }

    let mut uri = URI::new();

    uri.find(&buf_utf8);

    if uri.get() == &None {
        return Ok((StatusCode::BadRequest, None));
    };

    let requested_content = fs::read_to_string(format!(
        "{ABSOLUTE_STATIC_CONTENT_PATH}/{}",
        uri.get().clone().unwrap()
    ));
    let response = match requested_content {
        Ok(file) => response_success(file),
        Err(_err) => response_404(),
    };

    Ok((StatusCode::OK, Some(response)))
}

fn handle_sync_connection(
    logfile: &Option<File>,
    stream: &mut TcpStream,
) -> Result<(StatusCode, Option<String>), ServerError> {
    let buf = read_stream(stream)?;

    let buf_utf8 = parse_utf8(&buf)?;

    if !logfile.is_none() {
        logfile
            .as_ref()
            .unwrap()
            .write_all(
                format!(
                    "REQUEST AT {}\nREQUEST IP ADDRESS: {}\nINFO: {}\n===========\n",
                    generate_unixtime(),
                    stream.local_addr().unwrap(),
                    buf_utf8
                )
                .as_bytes(),
            )
            .unwrap();
    }

    let cors = Cors::new(HashSet::from([HttpRequestMethod::GET]));

    if !cors.method_is_allowed(&buf_utf8) {
        return Ok((StatusCode::MethodNotAllowed, None));
    }

    let mut uri = URI::new();

    uri.find(&buf_utf8);

    if uri.get() == &None {
        return Ok((StatusCode::BadRequest, None));
    };

    let requested_content = fs::read_to_string(format!(
        "{ABSOLUTE_STATIC_CONTENT_PATH}/{}",
        uri.get().clone().unwrap()
    ));
    let response = match requested_content {
        Ok(file) => response_success(file),
        Err(_err) => response_404(),
    };

    Ok((StatusCode::OK, Some(response)))
}

pub fn start_server(unixtime: u64) {
    print_license_info();

    let listener = TcpListener::bind(format!("{ADDR}:{PORT}")).unwrap();

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

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        if MULTITHREADING {
            thread::spawn(move || {
                let handled = multithread_handle_connection(&mut stream);

                let response = handle_response(handled);

                stream.write(&response.as_bytes()).unwrap();
                stream.flush().unwrap();
            });
        } else {
            let handled = handle_sync_connection(&logfile, &mut stream);

            let response = handle_response(handled);

            stream.write(&response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
