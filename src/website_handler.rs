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

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path_buf) => {
                println!("Is public? : {}", path_buf.as_path().starts_with(&self.public_path));
                println!("path_buf : {}", path_buf.as_path().display());
                println!("public_path : {}", &self.public_path);

                // not ideal but this line is to canonicalize the public path so it matches the same format as the path we are checking
                // this is needed on windows machines because sometimes \\? will be added to the start of the canonicalized path
                let public_path= fs::canonicalize(&self.public_path).unwrap().into_os_string().into_string().unwrap();

                if path_buf.as_path().starts_with(public_path) {
                    fs::read_to_string(path_buf).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
