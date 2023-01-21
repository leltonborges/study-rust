use std::borrow::Borrow;
use std::fmt::Formatter;
use std::time::SystemTime;

use actix_web::{App, get, http::header::ContentType, HttpRequest, HttpResponse, HttpServer, post, Responder, Result, web};
use actix_web::http::{StatusCode};
use actix_web::web::{Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Serialize)]
struct People {
    nome: String,
    documeto: String,
    data: String,
}

// #[tokio::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(data)
        // .service(data)
    })
        .bind(("localhost", 9080))?
        .run()
        .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}", name)
}

#[post("/body")]
// async fn data(data: web::Json<People>) -> Result<Json<People>> {
// async fn data(data: web::Json<People>) -> Result<impl Responder> {
async fn data(data: Json<People>, req: HttpRequest) -> HttpResponse {
    let obj = People { nome: data.nome.to_uppercase(), documeto: data.documeto.to_string(), data: data.data.to_string() };
    HttpResponse::Ok()
        .status(StatusCode::BAD_REQUEST)
        .insert_header(("Authorization", "Bearer:asdfsdfsadfsdf"))
        .insert_header(("method", req.method().to_string()))
        .content_type(ContentType::json())
        .json(obj)
}

