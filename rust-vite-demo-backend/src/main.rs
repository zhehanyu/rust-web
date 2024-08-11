use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct QueryResult {
    message: String,
}

#[get("/query")]
async fn query() -> impl Responder {
    println!("query !");
    let result = QueryResult {
        message: "查询结果".to_string(),
    };
    HttpResponse::Ok().json(result)
}

#[get("/action1")]
async fn action1() -> impl Responder {
    println!("action1 !");
    let result = QueryResult {
        message: "操作1已执行".to_string(),
    };
    HttpResponse::Ok().json(result)
}

#[get("/action2")]
async fn action2() -> impl Responder {
    println!("action2 !");
    let result = QueryResult {
        message: "操作2已执行".to_string(),
    };
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start ........................");
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // 允许任何域
                    .allow_any_method() // 允许任何HTTP方法
                    .allow_any_header() // 允许任何Header
            )
            .service(query)
            .service(action1)
            .service(action2)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
