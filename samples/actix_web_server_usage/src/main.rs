mod generic_handlers;
use runautils::actix_server_util::{serve_requests, Route};

use crate::generic_handlers::{boxed_get_req, boxed_post_handler, boxed_websocket_handler};
use crate::health_calls::boxed_health;

const ROUTES_LIST: &[Route] = &[
    Route {
        path: "/get_req",
        get_handler: Some(boxed_get_req),
        post_handler: None,
        websocket_handler: None,
    },
    Route {
        path: "/post_req",
        get_handler: None,
        post_handler: Some(boxed_post_handler),
        websocket_handler: None,
    },
    Route {
        path: "/ws",
        get_handler: None,
        post_handler: None,
        websocket_handler: Some(boxed_websocket_handler),
    },
];

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let routes = ROUTES_LIST.to_vec();
    serve_requests(routes).await
}
