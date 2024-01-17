use env_logger::Env;
use std::string;

use actix_cors::Cors;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder, middleware::Logger};
// use die

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

mod drawings;
mod models;
mod schema;
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }





#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(drawings::submit)
            .service(drawings::echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
