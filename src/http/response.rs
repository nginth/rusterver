/* response - A module containing the HttpResponse struct and related methods.
 *      HttpResponse represents a HTTP 1.1 response.
 *      The main method on HttpResponse is get_byte_response(), which is used
 *  to write a byte vector to the TcpStream in the server driver (main.rs).
 */
use std::vec::Vec;
use std::string::String;

pub struct HttpResponse<'a> {
    version: &'a str,
    code: &'a str,
    reason: &'a str,
    headers: Vec<&'a str>,
}

impl<'a> HttpResponse<'a> {
    pub fn new(code: &'a str, reason: &'a str) -> HttpResponse<'a> {
        let mut ret = HttpResponse 
            { 
                version: "HTTP/1.1", 
                code: code,
                reason: reason,
                headers: Vec::new(),
            };

            ret
    }

    pub fn get_reason(&self) -> &'a str {
        &self.reason
    }

    pub fn get_code(&self) -> &'a str {
        self.code
    }

    pub fn get_version(&self) -> &'a str {
        &self.version
    }

    /* Get the response that will be written to the TCPStream 
     * Returns an immutable reference to a byte slice.
     */
    pub fn get_byte_response<'b>(&'b self, mut byte_vector: &'b mut Vec<u8>) -> &[u8] {
        byte_vector.clear();
        self.construct_response(byte_vector);
        
        &byte_vector[..]
    }

    fn construct_response<'b>(&'b self, mut byte_vector: &'b mut Vec<u8>) {
        const SPACE: u8 = b' ';
        const CRLF: &'static [u8; 2] = b"\r\n";

        /* Construct response */
        byte_vector.extend_from_slice(self.version.as_bytes());
        byte_vector.push(SPACE);
        byte_vector.extend_from_slice(self.code.as_bytes());
        byte_vector.push(SPACE);
        byte_vector.extend_from_slice(self.reason.as_bytes());
        byte_vector.extend_from_slice(CRLF);

        println!("{}", String::from_utf8_lossy(byte_vector));
    }
}


