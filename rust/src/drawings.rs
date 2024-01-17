use crate::{models::*, DbPool, schema::drawings};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::RunQueryDsl;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/submit")]
async fn submit(pool: web::Data<DbPool>,req_body: web::Json<DrawingPayload>) -> impl Responder {
    let req_body = req_body.into_inner();
    let line_data = LineData::from_drawing_request(req_body.clone());

    if !line_data.validate_line() {
        return HttpResponse::BadRequest().json("Invalid line data!");
    }

    // Now we know we have a valid piece of line art, create a NewDrawing for insertion into the database
    let db_drawing = NewDrawing {
        lines: &req_body.data,
        artist: &req_body.artist,
        created_at: chrono::Local::now().naive_local(),
    };

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    diesel::insert_into(drawings::table)
        .values(&db_drawing)
        .execute(&mut conn)
        .expect("Error saving new drawing");


    HttpResponse::Ok().json("Success!")
}
