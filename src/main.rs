use std::fmt::Display;

use velen::{
    models::server_models::{Request, Response},
    server::create_server,
};
fn main() {
    let mut server = create_server();

    server.serve_static("./public");

    server.get("/get", get_handler);

    server.listen("127.0.0.1", 3333, |port| {
        println!("Application is running at port {:?}", port)
    });
}

fn get_handler(_: Request, mut res: Response) {
    res.set_status_code(200);
    res.set_header("Content-Type", "application/json");
    struct GoodResponse {
        message: String,
    }
    impl Display for GoodResponse {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{{\"message\": \"{}\"}}", self.message)
        }
    }
    let good_response = GoodResponse {
        message: "I'm good, thank you very much Sir!".to_string(),
    };
    res.send(good_response.to_string());
}
