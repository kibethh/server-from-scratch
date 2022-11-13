use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse the request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) -> Response {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!(
                                "Received a request from {}",
                                String::from_utf8_lossy(&buffer)
                            );

                            // Try into usage-Annotations are required
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // dbg!(request);
                                    // Response::new(
                                    //     StatusCode::Ok,
                                    //     Some("<h1>It Works!!!</h1>".to_string()),
                                    // )
                                    handler.handle_request(&request)
                                    // write!(stream, "{}", response);
                                    // response.send(&mut stream);
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                    // Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection:{}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }

            // let res = listener.accept();
            // if res.is_err() {
            //     continue;
            // }
            // let (stream, addr) = res.unwrap();
        }
    }
}
