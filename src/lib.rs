mod impls;
mod models;
mod utils;
pub mod server {
    use crate::models::server_models::Server;
    use std::collections::HashMap;

    pub fn create_server() -> Server {
        return Server {
            stream: None,
            request_table: HashMap::new(),
        };
    }
}
