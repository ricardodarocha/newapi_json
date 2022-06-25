use actix_web::{App, HttpServer};
use std::env;
mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let port = &args[1];
    let port: u16 = match port.parse() {
        Ok(number) => {
            number
        },
        Err(_) => {
            8000
        },
    };
    
    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
            .service(routes::info)
            
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}   

// cargo run -- {port}