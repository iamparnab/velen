# Velen
### Just another HTTP framework
### How to use

#### Import
```rust
use velen::server::create_server;
use velen::models::server_models::{Request, Response};
```

#### Create server instance
```rust
let mut server = create_server();
```

#### Register endpoints
```rust
server.get("/get", get_handler);
server.post("/add", post_handler);
```

#### Start listening
```rust
server.listen("127.0.0.1", 3333, server_start_handler);
```

#### Example handlers
```rust
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
```

#### Read Query Params

```rust
req.query_params.get("user_id").unwrap();
```

#### Read Request Header
```rust
req.headers.get("x-custom-header").unwrap();
```

#### Read Request Body
```rust
 /* Velen does not deserialize request payload. Use of Serde is recommended. */

 req.body;
 ```

 #### Set Response Status
```rust
res.set_status_code(200);
```

#### Set Response header
```rust
res.set_header("Content-Type", "application/json");
```

#### Set Response payload
```rust
res.send("{\"status\":\"ok\"}");
```


#### Limitations
- OPTIONS not implemented
- Only application/json is supported, so don't try to upload files 
- During sending a response `set_status_code` has to be called before any `set_header`. That is how response is actually sent to client.
#### TODO
- Fix above limitations
