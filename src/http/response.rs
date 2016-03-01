use std::string::String;
use std::vec::Vec;

pub struct HttpResponse {
    version: String,
    code: u32,
    reason: String,
    headers: Vec<String>,
}

impl HttpResponse {
    pub fn new(version: &str, code: u32, reason: &str) -> HttpResponse{
        let mut h = HttpResponse 
            { 
                version: version.to_string(), 
                code: code,
                reason: reason.to_string(),
                headers: Vec::new(),
            };
            
            h.headers.push("header1".to_string());
            h
    }

}