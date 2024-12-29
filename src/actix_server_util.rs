use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerContext {
    pub work_dir: String,
    pub port: String,
    pub dependencies: Arc<Box<dyn Any + Send + Sync>>,
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
        ) -> Pin<Box<dyn Future<Output = HttpResponse>>>,
    >,
    pub websocket_handler: Option<
        fn(
            actix_web::HttpRequest,
            actix_web::web::Payload,
        ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, actix_web::Error>>>>,
    >,
}

pub async fn serve_requests (
    routes_list: Vec<Route>,
    tmp_work_dir: String,
    tmp_port: String,
    server_context: Arc<Box<dyn Any + Send + Sync>>,
) -> std::io::Result<()> {
    let host_addr = format!("127.0.0.1:{}", tmp_port);

    println!("Starting server and serving on http://{}", host_addr);

    HttpServer::new(move || {
        let routes_list = routes_list.clone();
        let server_context = server_context.clone();
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .max_age(3600);

        App::new().wrap(cors).configure(|cfg| {
            for route in routes_list {
                if let Some(get_handler) = route.get_handler {
                    cfg.service(web::resource(route.path).route(web::get().to(get_handler)));
                }

                if let Some(post_handler) = route.post_handler {
                    let server_context = server_context.clone();
                    cfg.service(web::resource(route.path).route(web::post().to(
                        move |body: web::Json<String>| {
                            let server_context = server_context.clone();
                            async move { post_handler(body, route.path, server_context).await }
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
