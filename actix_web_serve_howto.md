

### define a routes list
```rust
const ROUTES_LIST: &[Route] = &[

    // GET REQUEST HANDLER ENTRY
    Route {
        path: "/get_req",
        get_handler: Some(boxed_get_req),
        post_handler: None,
        websocket_handler: None,
    },

    // POST REQUEST HANDLER ENTRY
    Route {
        path: "/post_req",
        get_handler: None,
        post_handler: Some(boxed_post_handler),
        websocket_handler: None,
    },

    // WEB SOCKET HANDLER ENTRY
    Route {
        path: "/ws",
        get_handler: None,
        post_handler: None,
        websocket_handler: Some(boxed_websocket_handler),
    },
];
```

### GET handler example
```rust

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, HttpResponse};
use actix_web_actors::ws;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

pub async fn get_req() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "generic http get handler" }))
}

pub fn boxed_get_req() -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(get_req())
}
```

### POST handler example
```rust

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, HttpResponse};
use actix_web_actors::ws;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

pub async fn post_req(body: web::Json<String>, path: &'static str) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "received": *body, "path": path }))
}

pub fn boxed_post_handler(
    body: web::Json<String>,
    path: &'static str,
) -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(post_req(body, path))
}
```

### WEB SOCKET handler example
```rust
use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, HttpResponse};
use actix_web_actors::ws;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

pub async fn websocket_handler(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WebSocketActor, &req, stream)
}

pub fn boxed_websocket_handler(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
) -> Pin<Box<dyn Future<Output = Result<HttpResponse, actix_web::Error>>>> {
    Box::pin(websocket_handler(req, stream))
}

struct WebSocketActor;

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mut elapsed = 0;
        ctx.run_interval(Duration::from_secs(1), move |_, ctx| {
            elapsed += 1;
            ctx.text(format!("{} second(s) elapsed", elapsed));
            if elapsed >= 10 {
                ctx.stop();
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Close(_)) = msg {
            ctx.stop();
        }
    }
}
```

### define your main function in main.rs
```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let routes = ROUTES_LIST.to_vec();
    serve_requests(routes).await
}
```