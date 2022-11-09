use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
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
                            match Request::try_from(&buffer[..]) {
                                Ok(_) => {}
                                Err(e) => println!("Failed to parse the request: {}", e),
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
