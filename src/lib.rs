use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, net::TcpListener};
use server_models::{Response, Request, Server};
use server_utils::{get_method_path_query, parse_body};

pub mod server {
    use std::collections::HashMap;

    use crate::server_models::Server;

    pub fn create_server() -> Server {
        return Server {
            stream: None,
            request_table: HashMap::new()
        }
    }
}

pub mod server_utils {
    use std::collections::HashMap;

    use regex::Regex;
    pub fn get_method_path_query(content: &str) -> (String, String, HashMap<String, String>) {
        /* First extract method and path + query_params */
        let outer_re = Regex::new(r"^(GET|POST) (.+) HTTP").unwrap();
        let outer_caps = outer_re.captures(content).unwrap();

        let method = outer_caps.get(1).unwrap().as_str().to_string();
        let path_with_query = outer_caps.get(2).unwrap().as_str().to_string();

        /* Then extract path and query_param string*/
        let inner_re = Regex::new(r"(.*)\?(.*)").unwrap();
        let inner_caps = inner_re.captures(&path_with_query);

        let path: String;
        let query_string: String;

        if let Some(caps) = inner_caps {
            path = caps.get(1).unwrap().as_str().to_string();
            query_string = caps.get(2).unwrap().as_str().to_string();
        } else {
            /* No Query String is present */
            path = path_with_query;
            query_string = String::from("");
        }

        let query_param_pairs = query_string.split("&");

        /* Then extrac individual Key and Values */
        let pair_re = Regex::new(r"(.*)=(.*)").unwrap();

        let mut query_map = HashMap::new();

        for pair in query_param_pairs {
            let pair_caps = pair_re.captures(pair);

            if let Some(caps) = pair_caps {
                let key = caps.get(1).unwrap().as_str().to_string();
                let value = caps.get(2).unwrap().as_str().to_string();

                query_map.insert(key, value);
            }
        }
        return (method, path, query_map);
    }
    pub fn parse_body(content: &str) -> String {
        let mut itr = content.split("\r\n\r\n");
        itr.next();
        let val = itr
            .next()
            .unwrap()
            .trim_end_matches(char::from(0))
            .to_string();
        return val;
    }
}

pub mod server_models {
    use std::{collections::HashMap, net::TcpStream};

    pub struct Server {
        pub stream: Option<TcpStream>,
        pub request_table: HashMap<String, fn(Request, Response) -> ()>,
    }
    pub struct Request {
        pub path: String,
        pub method: String,
        pub body: String,
        pub query_params: HashMap<String, String>,
    }
    pub struct Response {
        pub stream: TcpStream,
        pub code: i32,
    }
}


impl Server {
    pub fn listen(&mut self, host: &str, port: i32, handler: fn(Result<i32, io::Error>) -> ()) {
        let addr = format!("{host}:{port}");
        let listener = TcpListener::bind(addr);

        match listener {
            Ok(connection) => {
                handler(Ok(port));
                for connection in connection.incoming() {
                    match connection {
                        Ok(stream) => {
                            Server::handle_connection(&self, stream);
                        }
                        Err(err) => handler(Err(err)),
                    }
                }
            }
            Err(err) => handler(Err(err)),
        }
    }
    pub fn get(&mut self, path: &str, handler: fn(Request, Response) -> ()) {
        self.request_table.insert(format!("GET:{path}"), handler);
    }
    pub fn post(&mut self, path: &str, handler: fn(Request, Response) -> ()) {
        self.request_table.insert(format!("POST:{path}"), handler);
    }
    fn handle_connection(server: &Server, mut stream: TcpStream) {
        let mut req_body = [0; 1024];
        stream.read(&mut req_body).unwrap();

        let request_body = &String::from_utf8(req_body.to_vec()).unwrap();
        let (method, path, query_map) = get_method_path_query(request_body);
        let request_payload = parse_body(request_body);
        let handler = server.request_table.get(&format!("{method}:{path}"));

        match handler {
            Some(api_handler) => {
                let req = Server::build_request(method, path, request_payload, query_map);
                let res = Server::build_response(stream);

                api_handler(req, res);
            }
            None => {
                /* Handle 404 */
                stream
                    .write(
                        format!(
                            "{}\r\n{}\r\n\r\n{}",
                            "HTTP/1.1 404 NOT FOUND".to_string(),
                            "Content-Type: application/json".to_string(),
                            "{\"message\":\"Not found\"}".to_string()
                        )
                        .as_bytes(),
                    )
                    .unwrap();

                stream.flush().unwrap();
            }
        }
    }
    fn build_request(
        method: String,
        path: String,
        body: String,
        query_map: HashMap<String, String>,
    ) -> Request {
        return Request {
            method,
            path,
            body,
            query_params: query_map,
        };
    }
    fn build_response(stream: TcpStream) -> Response {
        return Response { stream, code: 200 };
    }
}

impl Response {
    pub fn set_status_code(&mut self, code: i32) {
        self.stream
            .write(format!("HTTP/1.1 {}", code).to_string().as_bytes())
            .unwrap();
    }
    pub fn set_header(&mut self, name: &str, value: &str) {
        self.stream
            .write(format!("\r\n{}: {}", name, value).to_string().as_bytes())
            .unwrap();
    }
    pub fn send(&mut self, body: String) {
        self.stream
            .write(format!("\r\n\r\n{}", body).to_string().as_bytes())
            .unwrap();
        self.stream.flush().unwrap();
    }
}
