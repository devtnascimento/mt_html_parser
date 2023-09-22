 
use std::net::TcpStream;
use std::rc::Rc;
use std::result::Result;
use std::io::Error;
use std::io::{Read, Write};


pub trait Http {
    fn get(&mut self, action: Option<&str>) -> Result<String, Error>;
}

pub struct HttpClient {
    host: String,
    stream: TcpStream
}

impl HttpClient {
    pub fn new(host: Rc<String>) -> Self {
        let stream = match TcpStream::connect(host.as_str()) {
            Ok(stream) => stream,
            Err(e) => {panic!("{e}")}
        };
        Self { 
            stream,
            host: host.to_string()
        }
    }
}

impl Http for HttpClient {
    fn get(&mut self, action: Option<&str>) -> Result<String, Error> {
        let request: String;
        match action {
            Some(s) => {request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", s, self.host);}
            None => {request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", self.host);}
        }

        self.stream.write(request.as_bytes())?;

        let mut response = String::new();
        self.stream.read_to_string(&mut response)?;

        Ok(response)
    }

}





