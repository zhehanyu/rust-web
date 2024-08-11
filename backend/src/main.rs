use std::fs;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Account {
    name: String,
    port1: u16,
    port2: u16,
}

#[derive(Serialize, Deserialize)]
struct Config {
    accounts: Vec<Account>,
}

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

#[get("/api/accounts")]
async fn get_accounts() -> impl Responder {
    let config_content = fs::read_to_string("config.json").expect("Failed to read config file");
    let config: Config = serde_json::from_str(&config_content).expect("Failed to parse config");
    HttpResponse::Ok().json(config.accounts)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Start ......................");
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
            .service(get_accounts)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}