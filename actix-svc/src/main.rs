use serde_derive::{Serialize, Deserialize};
// use serde_json::Result;

// Serialization
// https://serde.rs/impl-serialize.html

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    surname: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/json")]
async fn json() -> web::Json<User> {
    let user = User {
        name:       "Dima".to_string(),
        surname:    "Dv".to_string()
    };

    return web::Json(user);
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(json)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}