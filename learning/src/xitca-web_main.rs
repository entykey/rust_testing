use xitca_web::{handler::handler_service, route::get, App, HttpServer, Req};

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .at("/", get(handler_service(|req: Req| async move {
                let name = req.query::<String>("name").unwrap_or_else(|_| "World".to_owned());
                format!("Hello, {}!", name)
            })))
            .finish()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .wait()
}