use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    models::server_models::{Request, Response, Server},
    utils::server_utils::{get_method_path_query, parse_body},
};

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
    pub fn put(&mut self, path: &str, handler: fn(Request, Response) -> ()) {
        self.request_table.insert(format!("PUT:{path}"), handler);
    }
    pub fn delete(&mut self, path: &str, handler: fn(Request, Response) -> ()) {
        self.request_table.insert(format!("DELETE:{path}"), handler);
    }
    pub fn head(&mut self, path: &str, handler: fn(Request, Response) -> ()) {
        self.request_table.insert(format!("HEAD:{path}"), handler);
    }
    pub fn patch(&mut self, path: &str, handler: fn(Request, Response) -> ()) {
        self.request_table.insert(format!("PATCH:{path}"), handler);
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
