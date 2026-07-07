mod impls;
mod utils;

pub mod models;
pub mod server {
    use crate::models::server_models::Server;
    use std::collections::HashMap;

    pub fn create_server() -> Server {
        return Server {
            stream: None,
            request_table: HashMap::new(),
            request_table_private: HashMap::new(),
        };
    }
}
