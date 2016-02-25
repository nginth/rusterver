use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::io::{BufReader, BufRead, BufWriter};

fn handle_request(request: &TcpStream) {
    let stream = BufReader::new(request);
    for line in stream.lines() {
        println!("{}", line.unwrap());
    }    
}  

fn main() {
    let port = 8080;
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(ip_addr, port);
    let listener = TcpListener::bind(socket_addr).unwrap();

    println!("Server opened on port {}", socket_addr);
    for request in listener.incoming() {
        match request {
            Ok(request) => handle_request(&request),
            Err(e)      => println!("{}", e),
        }
    }

    drop(listener);
}
