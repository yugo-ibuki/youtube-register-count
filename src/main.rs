use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
    data: Vec<String>,
}

#[get("/")]
async fn hello() -> impl Responder {
    let response = ApiResponse {
        message: "Hello, JSON API!".to_string(),
        status: "success".to_string(),
        data: vec!["item1".to_string(), "item2".to_string(), "item3".to_string()],
    };
    web::Json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080");
    HttpServer::new(|| {
        App::new().service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}