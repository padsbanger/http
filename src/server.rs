use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }
    pub fn run(&self) {
        println!("Listening on address: {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(num) => match Request::try_from(&buffer as &[u8]) {
                            Ok(request) => {
                                dbg!(request);
                            }
                            Err(e) => println!("Failed to parse a request"),
                        },
                        Err(err) => {
                            println!("Error: {}", err);
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e)
                }
            }
        }

        //if(listener.)
    }
}
