use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    const GET_HOME: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    const GET_MAIN: &[u8; 10] = b"GET /build";

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(GET_HOME) {
        ("HTTP/1.1 200 OK\r\n\r\n", "html/index.html")
    } else if buffer.starts_with(GET_MAIN) {
        if buffer.starts_with(b"GET /build/class HTTP/1.1\r\n") {
            ("HTTP/1.1 200 OK\r\n\r\n", "html/build_class.html")

        } else if buffer.starts_with(b"GET /build/class HTTP/1.1\r\n") {
            ("HTTP/1.1 200 OK\r\n\r\n", "html/build_class.html")
        } else if buffer.starts_with(b"GET /build/subclass HTTP/1.1\r\n") {
            ("HTTP/1.1 200 OK\r\n\r\n", "html/build_subclass.html")
        } else if buffer.starts_with(b"GET /build/race HTTP/1.1\r\n") {
            ("HTTP/1.1 200 OK\r\n\r\n", "html/build_race.html")
        } else if buffer.starts_with(b"GET /build/spell HTTP/1.1\r\n") {
            ("HTTP/1.1 200 OK\r\n\r\n", "html/build_spell.html")
        } else if buffer.starts_with(b"GET /build/feat HTTP/1.1\r\n") {
            ("HTTP/1.1 200 OK\r\n\r\n", "html/build_feat.html")
        } else if buffer.starts_with(b"GET /build/background HTTP/1.1\r\n") {
            ("HTTP/1.1 200 OK\r\n\r\n", "html/build_background.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
        }
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
