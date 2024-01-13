use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
use actix_cors::Cors;


use serde::{Deserialize, Serialize};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// async fn manual_hello() -> impl Responder {
    //     HttpResponse::Ok().body("Hey there!")
    // }
    
#[derive(Deserialize, Serialize)]
struct DrawingData {
    data: String,
    name: String,
}

struct LineData {
    lines: Vec<Line>
}

impl LineData {
    /// Validates that each line segment in fact connects to one another sequentilly, beginning at 0,200 and ending at 600,200
    fn validate_line(&self) -> bool {
        let mut prev_x = 0.0;
        let mut prev_y = 200.0;
        for line in &self.lines {
            if line.x1 != prev_x || line.y1 != prev_y {
                return false;
            }
            prev_x = line.x2;
            prev_y = line.y2;
        }
        if prev_x != 600.0 || prev_y != 200.0 {
            return false;
        }
        true
    }
}

struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl DrawingData {
    fn get_line_data(&self) -> LineData {
        let lines: Vec<Line> = Vec::new();
        let mut line_data = LineData { lines };
        let lines = self.data.split(";");
        for line in lines {
            let mut coords = line.split(",");
            let x1 = coords.next().unwrap().parse::<f32>().unwrap();
            let y1 = coords.next().unwrap().parse::<f32>().unwrap();
            let x2 = coords.next().unwrap().parse::<f32>().unwrap();
            let y2 = coords.next().unwrap().parse::<f32>().unwrap();
            let line = Line { x1, y1, x2, y2 };
            line_data.lines.push(line);
        }
        line_data
    }
}



#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/submit")]
async fn submit(req_body: web::Json<DrawingData>) -> impl Responder {
    let data = req_body.data.clone();
    let name = req_body.name.clone();
    let drawing_data = DrawingData { data, name };
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
