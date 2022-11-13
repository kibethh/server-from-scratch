use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
pub struct WebsiteHandler {
    public_path: String,
}
impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&mut self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        // fs::canonicalize(path) - removes double dots for access to parent directories
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    // ok()-> checks if result is ok and converts it to an option of some(value)
                    // ok()-> Otherwise it converts it into a Nones
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {:?}", path);
                    None
                }
            }
            Err(_) => None,
        }
        // ok()-> checks if result is ok and converts it to an option of some(value)
        // ok()-> Otherwise it converts it into a Nones
        // fs::read_to_string(path).ok()
    }
}
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string()))
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),

                // Security vulnerability - Directory Traversal vulnerability
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
