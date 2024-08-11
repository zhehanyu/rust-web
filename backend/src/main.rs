use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct QueryParams {
    query: String,
}

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust!")
}

#[post("/api/execute")]
async fn execute(info: web::Json<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Function executed",
        "data": info.0
    }))
}

#[get("/api/query")]
async fn query(info: web::Query<QueryParams>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "result": format!("Query result for: {}", info.query)
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello world");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(hello)
            .service(execute)
            .service(query)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}