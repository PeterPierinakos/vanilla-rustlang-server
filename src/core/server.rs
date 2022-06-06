use crate::configuration::*;
use crate::constants::*;
use crate::core::time::generate_unixtime;
use crate::structs::uri::URI;
use crate::util::response::*;
use std::io::Write;
use std::str;
use std::thread;
use std::{fs, fs::File, fs::OpenOptions};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn async_handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    stream.read(&mut buf).unwrap();

    if !buf.starts_with(GET) {
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let mut uri = URI::new();

    uri.find(&str::from_utf8(&buf).unwrap().to_string());

    if uri.get() == &None {
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    };

    let requested_content = fs::read_to_string(format!(
        "{}/html/{}",
        ABSOLUTE_STATIC_CONTENT_PATH,
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
    let mut buf = [0; 1024];

    stream.read(&mut buf).unwrap();

    if !logfile.is_none() {
        logfile
            .as_ref()
            .unwrap()
            .write_all(
                format!(
                    "REQUEST AT {}\nINFO: {}\n===========\n",
                    generate_unixtime(),
                    str::from_utf8(&buf).unwrap()
                )
                .as_bytes(),
            )
            .unwrap();
    }

    if !buf.starts_with(GET) {
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let mut uri = URI::new();

    uri.find(&str::from_utf8(&buf).unwrap().to_string());

    if uri.get() == &None {
        let response = response_400();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    };

    let requested_content = fs::read_to_string(format!(
        "{}/html/{}",
        ABSOLUTE_STATIC_CONTENT_PATH,
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
    let listener = TcpListener::bind(format!("{}:{}", ADDR, PORT)).unwrap();

    let logfile = if SAVE_LOGS {
        Some(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("{}/{}.txt", ABSOLUTE_LOGS_PATH, unixtime))
                .expect("Unable to open logs"),
        )
    } else {
        None
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        if ASYNC {
            handle_thread(stream);
        } else {
            handle_sync_connection(&logfile, stream)
        }
    }
}

pub fn handle_thread(stream: TcpStream) {
    thread::spawn(|| {
        async_handle_connection(stream);
    });
}
