use std::net::TcpListener;

pub fn start_server(addr: String, port: i32, debug_info: bool) {
    let listener = TcpListener::bind(format!("{}:{}", addr, port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        if debug_info {
            println!("Connection established!");
        }
    }
}
