use std::string;

use actix_cors::Cors;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
// use die

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }



#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/submit")]
async fn submit(req_body: web::Json<DrawingRequest>) -> impl Responder {

    let drawing_data = req_body;
    let line_data = drawing_data.get_line_data();
    if !line_data.validate_line() {
        return HttpResponse::BadRequest().json("Invalid line data!");
    }

    HttpResponse::Ok().json("Success!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(submit)
            // .service(hello)
            .service(echo)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
