pub mod server {
    use std::collections::HashMap;

    use crate::models::server_models::Server;
    pub fn create_server() -> Server {
        return Server {
            stream: None,
            request_table: HashMap::new()
        }
    }
}