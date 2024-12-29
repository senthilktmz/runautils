use actix_web::{web, App, HttpResponse, HttpServer};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerContext {
    pub work_dir: String,
    pub port: String,
}

#[derive(Clone)]
pub struct Route {
    pub path: &'static str,
    pub get_handler: Option<fn() -> Pin<Box<dyn Future<Output = HttpResponse>>>>,
    pub post_handler: Option<
        fn(
            web::Json<String>,
            &'static str,
            Arc<ServerContext>,
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
    work_dir: String,
    tmp_port: String,
) -> std::io::Result<()> {
    println!("Starting server");

    let server_context = Arc::new(ServerContext {
        port: tmp_port,
        work_dir,
    });

    HttpServer::new(move || {
        let server_context = server_context.clone();
        let app = routes_list.iter().fold(App::new(), move |app, route| {
            let app = if let Some(get_handler) = route.get_handler {
                app.route(route.path, web::get().to(get_handler))
            } else {
                app
            };

            let app = if let Some(post_handler) = route.post_handler {
                let path = route.path;
                let server_context = server_context.clone();
                app.route(
                    route.path,
                    web::post().to(move |body: web::Json<String>| {
                        let server_context = server_context.clone();
                        async move { post_handler(body, path, server_context).await }
                    }),
                )
            } else {
                app
            };

            if let Some(ws_handler) = route.websocket_handler {
                app.route(route.path, web::get().to(ws_handler))
            } else {
                app
            }
        });

        app
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
