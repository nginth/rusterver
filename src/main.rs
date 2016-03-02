/* rusterver - a basic memory safe multithreaded webserver
 * While it is memory safe, it is completely unsecure, so beware
 * of actually using it as a web server.
 */
use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::str;
use std::string::String;
use std::thread;

mod http;
use http::request::HttpRequest;
use http::response::HttpResponse;

fn do_response(stream: &TcpStream) {
    let http_request: HttpRequest = parse_request(stream);
    send_response(stream);
} 

fn parse_request(stream: &TcpStream) -> HttpRequest {
    const CRLF: &'static str = "\r\n";
    let mut reader = BufReader::new(stream);
    let mut request_string = String::new();

    HttpRequest::new(reader.lines())
} 

fn send_response(stream: &TcpStream) {
    // send response
    let mut writer = BufWriter::new(stream);
    let response = HttpResponse::new("200", "OK");
    let mut buf = Vec::new();
    let byte_response: &[u8] = response.get_byte_response(&mut buf);

    writer.write(byte_response).unwrap();
}

fn main() {
    let port = 8080;
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(ip_addr, port);
    let listener = TcpListener::bind(socket_addr).unwrap();

    println!("Server opened on port {}", socket_addr);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    do_response(&stream)
                });
            },
            Err(e)      => println!("{}", e),
        }
    }

    drop(listener);
}
