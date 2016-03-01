use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::str;
use std::thread;

mod http;
use http::HttpRequest;

fn handle_request(request: &TcpStream) {
    let mut stream = BufReader::with_capacity(1024, request);
    
    // block in order to borrow stream again for consume()
    let len = {
        // grab 1024 bytes from the stream
        let buf = stream.fill_buf().unwrap();
        let http_req = match str::from_utf8(&buf) {
            Ok(string) => string,
            Err(e) => panic!("cannot convert buf into utf8: {}", e),
        };

        println!("{}", http_req);
        buf.len()
    };
    // let the BufReader know not to read those bytes again
    stream.consume(len);

    // send response
    let mut stream = BufWriter::new(request);
    stream.write(b"hello\n").unwrap();
}  

fn main() {
    let port = 8080;
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(ip_addr, port);
    let listener = TcpListener::bind(socket_addr).unwrap();

    println!("Server opened on port {}", socket_addr);
    for request in listener.incoming() {
        match request {
            Ok(request) => {
                thread::spawn(move || {
                    handle_request(&request)
                });
            },
            Err(e)      => println!("{}", e),
        }
    }

    drop(listener);
}
