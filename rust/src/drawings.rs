use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::models::*;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/submit")]
async fn submit(req_body: web::Json<DrawingRequest>) -> impl Responder {
    let line_data = LineData::from_drawing_request(req_body.into_inner());

    if !line_data.validate_line() {
        return HttpResponse::BadRequest().json("Invalid line data!");
    }

    HttpResponse::Ok().json("Success!")
}