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
}
