use crate::http::{
    response::{self, Response},
    Request, StatusCode, request::{self, ParseError},
};
use std::convert::TryFrom;
use std::{io::Read, io::Write, net::TcpListener};

pub trait  Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        Response::new(StatusCode::BadRequest, None)
    }
}


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Server {
        return Server { addr };
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut tcp_stream, _)) => {
                    let mut buffer = [0; 1024];
                    match tcp_stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request {}", String::from_utf8_lossy(&buffer));
                           let response =  match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    println!("{:?}", request);
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    println!("Failed to parse the rquest {}", e);
                                    handler.handle_bad_request(&e)
                                }
                            };


                            if let Err(e) = response.send(&mut tcp_stream) {
                                println!("Failed to send response {}", e)
                            }
                        }
                        Err(e) => println!("Failed to read from connection {}", e),
                    }
                }
                Err(error) => println!("Faild to accept connection {}", error),
            }
        }
    }
}
