use chrono::NaiveDateTime;
use diesel::{prelude::*, sql_types::Timestamp, Insertable};
use serde::{Deserialize, Serialize};
use crate::schema::drawings;

// use crate::sc

#[derive(Queryable, Selectable)]
#[diesel(table_name = drawings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Drawing {
    pub id: i32,
    pub lines: String,
    pub artist: String,
    pub created_at: NaiveDateTime,
}

impl Drawing {

}

#[derive(Insertable)]
#[diesel(table_name = drawings)]
pub struct NewDrawing<'a> {
    pub lines: &'a str,
    pub artist: &'a str,
    pub created_at: NaiveDateTime,
} 

impl NewDrawing<'_> {
    
}

#[derive(Deserialize, Serialize)]
pub struct DrawingPayload {
    data: String,
    artist: String,
}

// impl DrawingRequest {
// fn to_drawing_data(&self) -> DrawingData {
//     let id = 0;
//     let data = self.data.clone();
//     let artist = self.artist.clone();
//     let created_at = chrono::Local::now().naive_local();
//     DrawingData {
//         id,
//         data,
//         artist,
//         created_at,
//     }
// }
// }

// struct DrawingData {
//     id: u32,
//     data: String,
//     artist: String,
//     created_at: NaiveDateTime,
// }

pub struct LineData {
    lines: Vec<Line>,
}

impl LineData {
    /// Validates that each line segment in fact connects to one another sequentilly, beginning at 0,200 and ending at 600,200
    pub fn validate_line(&self) -> bool {
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

    pub fn from_drawing_request(req: DrawingPayload) -> LineData {
        let lines: Vec<Line> = Vec::new();
        let mut line_data = LineData { lines };
        let lines = req.data.split(";");
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

pub struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}
