
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ServerStateStore {
    pub state: Arc<Mutex<HashMap<String, Arc<Box<dyn Any + Send + Sync>>>>>,//HashMap<String, String>,
}

impl ServerStateStore {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[derive(Clone)]
pub struct Route {
    pub path: &'static str,
    pub get_handler: Option<fn() -> Pin<Box<dyn Future<Output = HttpResponse>>>>,
    pub post_handler: Option<
        fn(
            web::Json<String>,
            &'static str,
            Arc<Box<dyn Any + Send + Sync>>,
            Arc<Mutex<ServerStateStore>>,
        ) -> Pin<Box<dyn Future<Output = HttpResponse>>>,
    >,
    pub websocket_handler: Option<
        fn(
            actix_web::HttpRequest,
            actix_web::web::Payload,
        ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, actix_web::Error>>>>,
    >,
}


pub async fn serve_requests(
    routes_list: Vec<Route>,
    tmp_work_dir: String,
    tmp_port: String,
    server_context: Arc<Box<dyn Any + Send + Sync>>,
) -> std::io::Result<()> {
    let host_addr = format!("127.0.0.1:{}", tmp_port);

    // Create the shared server state store
    let server_state_store = Arc::new(Mutex::new(ServerStateStore::new()));

    println!("Starting server and serving on http://{}", host_addr);

    HttpServer::new(move || {
        let routes_list = routes_list.clone();
        let server_context = server_context.clone();
        let server_state_store = server_state_store.clone(); // Clone Arc for each thread

        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .max_age(3600);

        App::new().wrap(cors).configure(move |cfg| {
            for route in routes_list.clone() {
                if let Some(get_handler) = route.get_handler {
                    cfg.service(web::resource(route.path).route(web::get().to(get_handler)));
                }

                if let Some(post_handler) = route.post_handler {
                    let server_context = server_context.clone();
                    let server_state_store = server_state_store.clone();

                    cfg.service(web::resource(route.path).route(web::post().to(
                        move |body: web::Json<String>| {
                            let server_context = server_context.clone();
                            let server_state_store = server_state_store.clone();

                            async move {
                                post_handler(body, route.path, server_context, server_state_store)
                                    .await
                            }
                        },
                    )));
                }

                if let Some(ws_handler) = route.websocket_handler {
                    cfg.service(web::resource(route.path).route(web::get().to(ws_handler)));
                }
            }
        })
    })
        .bind(host_addr)?
        .run()
        .await
}
