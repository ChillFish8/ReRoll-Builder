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

fn get_html_file(filename: &str) -> String {
    return format!("html/{}.html", filename);
}

fn handle_connection(mut stream: TcpStream) {
    const GET_HOME: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    const GET_MAIN: &[u8; 10] = b"GET /build";
    const HTTP_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
    const HTTP_NOTFOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(GET_HOME) {
        (HTTP_OK, get_html_file("index"))
    } else if buffer.starts_with(GET_MAIN) {
        if buffer.starts_with(b"GET /build/class HTTP/1.1\r\n") {
            (HTTP_OK, get_html_file("build_class"))
        } else if buffer.starts_with(b"GET /build/subclass HTTP/1.1\r\n") {
            (HTTP_OK, get_html_file("build_subclass"))
        } else if buffer.starts_with(b"GET /build/race HTTP/1.1\r\n") {
            (HTTP_OK, get_html_file("build_race"))
        } else if buffer.starts_with(b"GET /build/spell HTTP/1.1\r\n") {
            (HTTP_OK, get_html_file("build_spell"))
        } else if buffer.starts_with(b"GET /build/feat HTTP/1.1\r\n") {
            (HTTP_OK, get_html_file("build_feat"))
        } else if buffer.starts_with(b"GET /build/background HTTP/1.1\r\n") {
            (HTTP_OK, get_html_file("build_background"))
        } else {
            (HTTP_NOTFOUND, get_html_file("404"))
        }
    } else {
        (HTTP_NOTFOUND, get_html_file("404"))
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
