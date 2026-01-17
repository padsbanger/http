use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

use crate::http::ParseError;

use super::http::{Request, Response, StatusCode};

pub struct Server {
    address: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }
    pub fn run(&self, mut handler: impl Handler) {
        println!("Listening on address: {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    let response = match stream.read(&mut buffer) {
                        Ok(num) => match Request::try_from(&buffer as &[u8]) {
                            Ok(request) => handler.handle_request(&request),
                            Err(e) => handler.handle_bad_request(&e),
                        },
                        Err(err) => Response::new(StatusCode::BadRequest, None),
                    };

                    if let Err(e) = response.send(&mut stream) {}
                }
                Err(e) => {
                    println!("Error: {}", e)
                }
            }
        }

        //if(listener.)
    }
}
