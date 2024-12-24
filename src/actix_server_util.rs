use actix_web::{web, App, HttpResponse, HttpServer};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct Route {
    pub path: &'static str,
    pub get_handler: Option<fn() -> Pin<Box<dyn Future<Output = HttpResponse>>>>,
    pub post_handler:
        Option<fn(web::Json<String>, &'static str) -> Pin<Box<dyn Future<Output = HttpResponse>>>>,
    pub websocket_handler: Option<
        fn(
            actix_web::HttpRequest,
            actix_web::web::Payload,
        ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, actix_web::Error>>>>,
    >,
}

pub async fn serve_requests(routes_list: Vec<Route>) -> std::io::Result<()> {
    println!("Starting server");

    HttpServer::new(move || {
        let app = routes_list.iter().fold(App::new(), |app, route| {
            let app = if let Some(get_handler) = route.get_handler {
                app.route(route.path, web::get().to(get_handler))
            } else {
                app
            };

            let app = if let Some(post_handler) = route.post_handler {
                let path = route.path;
                app.route(
                    route.path,
                    web::post().to(move |body: web::Json<String>| async move {
                        post_handler(body, path).await
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
