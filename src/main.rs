use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::io::{Read, Write};
use std::string::String;

fn handle_request(mut request: TcpStream) {
    let mut buf = String::new();
    
    let result = request.read_to_string(&mut buf);
    match result {
        Ok(_) => println!("{}", buf),
        Err(_) => { return },
    }
    request.write_all(b"Hello world!");
}  

fn main() {
    let port = 8080;
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(ip_addr, port);
    let listener = TcpListener::bind(socket_addr).unwrap();

    println!("Server opened on port {}", socket_addr);
    for request in listener.incoming() {
        match request {
            Ok(request) => handle_request(request),
            Err(e)      => println!("{}", e),
        }
    }
}
