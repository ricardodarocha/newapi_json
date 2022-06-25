use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::Hello;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/info")]
pub async fn info(req_body: String) -> impl Responder {
    let obj = Hello::new("Olá mundo".to_owned());
    Ok(web.json(obj))
}
