use crate::configuration::*;
use crate::core::time::generate_unixtime;
use crate::enums::methods::HttpRequestMethod;
use crate::structs::cors::Cors;
use crate::structs::uri::URI;
use crate::util::license::print_license_info;
use crate::util::response::*;
use crate::util::socket::handle_stream;
use std::collections::HashSet;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::{fs, fs::File, fs::OpenOptions};
use crate::util::socket::parse_utf8;

fn multithread_handle_connection(mut stream: TcpStream) {
    let buf = handle_stream(&mut stream);

    /* Handle connections that send no data. */
    /* === */


    if buf.is_none() {
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let buf = buf.unwrap();

    let buf_utf8 = parse_utf8(&buf);

    if buf_utf8.is_none() { 
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let buf_utf8 = buf_utf8.unwrap();

    /* === */

    let cors = Cors::new(HashSet::from([HttpRequestMethod::GET]));

    if !cors.method_is_allowed(&buf_utf8) {
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let mut uri = URI::new();

    uri.find(&buf_utf8);

    if uri.get() == &None {
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    };

    let requested_content = fs::read_to_string(format!(
        "{ABSOLUTE_STATIC_CONTENT_PATH}/{}",
        uri.get().clone().unwrap()
    ));
    let response = match requested_content {
        Ok(file) => response_success(file),
        Err(_err) => response_404(),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_sync_connection(logfile: &Option<File>, mut stream: TcpStream) {
    let buf = handle_stream(&mut stream);

    /* Handle connections that send no data. */
    /* === */

    if buf.is_none() {
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let buf = buf.unwrap();

    let buf_utf8 = parse_utf8(&buf);

    if buf_utf8.is_none() { 
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let buf_utf8 = buf_utf8.unwrap();

    /* === */

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
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let mut uri = URI::new();

    uri.find(&buf_utf8);

    if uri.get() == &None {
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    };

    let requested_content = fs::read_to_string(format!(
        "{ABSOLUTE_STATIC_CONTENT_PATH}/{}",
        uri.get().clone().unwrap()
    ));
    let response = match requested_content {
        Ok(file) => response_success(file),
        Err(_err) => response_404(),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
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
        let stream = stream.unwrap();

        if MULTITHREADING {
            handle_thread(stream);
        } else {
            handle_sync_connection(&logfile, stream)
        }
    }
}

pub fn handle_thread(stream: TcpStream) {
    thread::spawn(|| {
        multithread_handle_connection(stream);
    });
}
