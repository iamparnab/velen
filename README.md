# Velen
### Just another REST framework
### How to use

#### Import
<code>
use velen::server::create_server;
    
use velen::models::server_models::{Request, Response};
</code>

#### Create server instance
<code>
let mut server = create_server();
</code>

#### Register endpoints
<code>
server.get("/get", get_handler);

server.post("/add", post_handler);</code>

#### Start listening
<code>
server.listen("127.0.0.1", 3333, server_start_handler);</code>

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
</pre></code>

#### Read Query Params
<code>req.query_params.get("user_id").unwrap();</code>

#### Read Request Header
<code>req.headers.get("x-custom-header").unwrap();</code>

#### Read Request Body
<code>
 /* Velen does not deserialize request payload. Use of Serde is recommended. /*

 req.body;</code>

 #### Set Response Status
<code>res.set_status_code(200);</code>

#### Set Response header
<code>res.set_header("Content-Type", "application/json");</code>

#### Set Response payload
<code>res.send("{\"status\":\"ok\"}");</code>


#### Limitations
- OPTIONS not implemented
- Only application/json is supported, so don't try to upload files 
- During sending a response `set_status_code` has to be called before any `set_header`. That is how response is actually sent to client.
- No multithreading
#### TODO
- Fix above limitations
