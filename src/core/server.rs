use crate::configuration::*;
use crate::constants::*;
use crate::core::time::generate_unixtime;
use std::io::Write;
use std::str;
use std::{fs, fs::File, fs::OpenOptions};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_connection(logfile: &Option<File>, mut stream: TcpStream) {
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
        let page_400 = fs::read_to_string(format!("{}html/400.html", ABSOLUTE_STATIC_CONTENT_PATH))
            .expect(
                format!(
                    "400 HTML page doesn't exist ('{}html/400.html')",
                    ABSOLUTE_STATIC_CONTENT_PATH
                )
                .as_str(),
            );
        let response = format!(
            "HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}",
            page_400.len(),
            page_400
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let page_404_content =
        fs::read_to_string(format!("{}html/404.html", ABSOLUTE_STATIC_CONTENT_PATH)).expect(
            format!(
                "404 HTML page doesn't exist ('{}html/400.html')",
                ABSOLUTE_STATIC_CONTENT_PATH
            )
            .as_str(),
        );

    let requested_content =
        fs::read_to_string(format!("{}html/index.html", ABSOLUTE_STATIC_CONTENT_PATH));
    let response = match requested_content {
        Ok(file) => format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            file.len(),
            file
        ),
        Err(_err) => format!(
            "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
            page_404_content.len(),
            page_404_content
        ),
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
                .open(format!("{}{}.txt", ABSOLUTE_LOGS_PATH, unixtime))
                .expect("Unable to open logs"),
        )
    } else {
        None
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(&logfile, stream);
    }
}
