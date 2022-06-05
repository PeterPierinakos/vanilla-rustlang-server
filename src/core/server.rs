use crate::configuration::*;
use crate::core::time::generate_unixtime;
use std::io::Write;
use std::{fs::File, fs::OpenOptions};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_connection(logfile: &Option<File>, mut stream: TcpStream) {
    let mut buf = String::new();

    stream.read_to_string(&mut buf).unwrap();

    if !logfile.is_none() {
        logfile
            .as_ref()
            .unwrap()
            .write_all(
                format!(
                    "REQUEST AT {}\nINFO: {}\n===========\n",
                    generate_unixtime(),
                    buf
                )
                .as_bytes(),
            )
            .unwrap();
    }
}

pub fn start_server(unixtime: u64) {
    let listener = TcpListener::bind(format!("{}:{}", ADDR, PORT)).unwrap();

    let logfile = if SAVE_LOGS {
        Some(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("{}{}.txt", LOGS_PATH, unixtime))
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
