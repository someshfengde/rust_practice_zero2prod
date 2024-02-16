use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
async fn makeit() -> impl Responder {
    HttpResponse::Ok().body("I am alive!")
}
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(greet))
            .route("/hello", web::get().to(makeit))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();
    // .await
    Ok(server)
}
