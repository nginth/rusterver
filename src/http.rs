use std::string::String;

pub struct HttpRequest {
    method: String
}

impl HttpRequest {
    pub fn print_method(&self) {
        println!("{}", self.method);
    }

    pub fn new<'a>(met: String) -> HttpRequest {
        HttpRequest { method: met }
    }
}

pub fn test_print_method() {
    let http: HttpRequest = HttpRequest::new("hello!".to_string());
    http.print_method();
}