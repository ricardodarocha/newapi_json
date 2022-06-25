use actix_web::{get, post, web, HttpResponse, Responder};
mod models;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/hello")]
pub async fn echo(req_body: String) -> impl Responder {
    let obj = MyObj {
        name: name.to_string(),
    };
    HttpResponse::Ok(web.json(obj))
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}