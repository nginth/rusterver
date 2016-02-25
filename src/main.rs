use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::io::{BufReader, BufRead, BufWriter, Read};
use std::str;

fn handle_request(request: &TcpStream) {
    let mut stream = BufReader::with_capacity(1024, request);
    println!("here");
    
    let len = {
        let buf = stream.fill_buf().unwrap();
        println!("not here");
        let http_req = match str::from_utf8(&buf) {
            Ok(string) => string,
            Err(e) => panic!("cannot convert buf into utf8: {}", e),
        };
        println!("{}", http_req);
        buf.len()
    };
    
    stream.consume(len);
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
