# Velen
### Just another REST framework
### How to use

#### Import
<code>
<pre>
use velen::{server::create_server, server_models::{Request, Response}};
</pre>
</code>

#### Create server instance
<code>
<pre>
let mut server = create_server();
</pre>
</code>

#### Register endpoints
<code>
<pre>
server.get("/get", get_handler);
server.post("/add", post_handler);
</pre>
</code>

#### Start listening
<code>
<pre>
server.listen("127.0.0.1", 3333, server_start_handler);
</pre>
</code>

#### Example handlers
<code>
<pre>
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
fn post_handler(req: Request, mut res: Response) {
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
        message: format!("Received query parameters x={:?}, y={:?}", req.query_params.get("x").unwrap(), req.query_params.get("y").unwrap()),
    };
    res.send(good_response.to_string());
}
</pre>
</code>
