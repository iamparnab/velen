pub mod server_models {
    use std::{collections::HashMap, net::TcpStream};

    pub struct Server {
        pub stream: Option<TcpStream>,
        pub request_table: HashMap<String, fn(Request, Response) -> ()>,
        pub request_table_private:
            HashMap<String, Box<dyn Fn(Request, Response) + Send + Sync + 'static>>,
    }
    pub struct Request {
        pub path: String,
        pub method: String,
        pub body: String,
        pub query_params: HashMap<String, String>,
        pub headers: HashMap<String, String>,
    }
    pub struct Response {
        pub stream: TcpStream,
        pub code: i32,
    }
}
