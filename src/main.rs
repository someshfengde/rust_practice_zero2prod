use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn makeit() -> impl Responder {
    HttpResponse::Ok().body("I am alive!")
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().route("/",web::get().to(greet))
        .route("/hello", web::get().to(greet))
        .route("/health_check" ,web::get().to(makeit))
    })
    .bind("127.0.0.1:8123")?
    .run()
    .await
}