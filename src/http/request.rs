use std::string::String;
use std::io::{Lines, BufReader};
use std::net::TcpStream;

pub struct HttpRequest<'a> {
    method: &'a str,
    path: &'a str,
    version: &'a str
}

impl<'a> HttpRequest<'a> {
    pub fn print_method(&self) {
        println!("{}", self.method);
    }

    pub fn new_from_parts
        (m: &'a str, p: &'a str, v: &'a str) -> HttpRequest<'a> {
        HttpRequest 
        { method: m, path: p, version: v}
    }

    /* TODO: handle request bodies */
    pub fn new(lines: Lines<BufReader<&TcpStream>>) -> HttpRequest<'a> { 
        let mut req = HttpRequest {method: "x", path: "y", version: "z"};
        //figure out how lines.next() works

        for line in lines {
            let line_string = line.unwrap();
            let slice = &line_string[..];
            /* if slice is empty, we've hit the end of the request (unless there's a body) */
            if slice == "" { break; };
            parse_line(slice, &mut req);
        }

        req
    }
}


fn parse_line(slice: &str, mut request: &mut HttpRequest) {
}
