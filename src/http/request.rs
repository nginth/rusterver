use std::string::String;

pub struct HttpRequest {
    method: String,
    path: String,
    version: String
}

impl HttpRequest {
    pub fn print_method(&self) {
        println!("{}", self.method);
    }

    pub fn new_from_parts
        (m: &str, p: &str, v: &str) -> HttpRequest {
        HttpRequest 
        { method: m.to_string(), path: p.to_string(), version: v.to_string()}
    }

    pub fn new(request: String) {

    }
}
