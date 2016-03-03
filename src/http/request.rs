use std::string::String;
use std::io::{Lines, BufReader, Error};
use std::net::TcpStream;
use std::str::SplitWhitespace;

pub struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: Vec<String>
}

impl HttpRequest {
    pub fn get_method(&self) -> &String {
        &self.method
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    /* TODO: handle request bodies 
     * TODO: add better error handling
     */
    pub fn new(lines: &mut Lines<BufReader<&TcpStream>>) -> HttpRequest { 
        let mut request = HttpRequest {
            method: String::new(), 
            path: String::new(), 
            version: String::new(), 
            headers: Vec::new()
        };
        
        let mut request_line = lines.next().unwrap().unwrap();
        parse_req_line(&mut request_line, &mut request);

        for line in lines {
            let line_string = line.unwrap();
            let slice = &line_string[..];
            /* if slice is empty, we've hit the end of the request (unless there's a body) */
            if slice == "" { break; };
            let mut s = String::new();
            s.push_str(slice);
            request.headers.push(String::from(s));
        }

        request
    }
}

pub fn parse_req_line(request_line: &mut str, request: &mut HttpRequest) {
    // definitely add error handling here
    let mut req_iterator = request_line.split_whitespace();
    request.method.push_str(req_iterator.next().unwrap());
    request.path.push_str(req_iterator.next().unwrap());
    request.version.push_str(req_iterator.next().unwrap());
}

