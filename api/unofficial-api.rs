use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json;
// use chrono::prelude::*;
use unofficial_api::{Canceled, Classes, Moved, Supplementary};

// fn get_jst_yyyymm() -> String {
//     let dt = FixedOffset::east(9 * 3600);
//     let jst_now = Utc::now().with_timezone(&dt);
//     jst_now.format("%Y%m").to_string()
// }

// yyyymmを1ヶ月巻き戻す処理
// fn minus_one_month(yyyymm: &str) -> String {
//     let (yyyy, mm) = yyyymm.split_at(4);
//     let (subtracted_yyyy, subtracted_mm);
//     if mm == "01" {
//         subtracted_yyyy = (yyyy.parse::<u16>().unwrap() - 1).to_string();
//         subtracted_mm = "12".to_string();
//     } else {
//         subtracted_yyyy = yyyy.to_string();
//         subtracted_mm = {
//             let m = (mm.parse::<u16>().unwrap() - 1).to_string();
//             match m.len() {
//                 1 => "0".to_string() + &m,
//                 2 => m,
//                 _ => panic!("Invalid mm"),
//             }
//         };
//     }
//     subtracted_yyyy + &subtracted_mm
// }

fn get_yyyymm() -> String {
    String::from("201912")
}

async fn get_classes() -> impl Responder {
    let yyyymm = get_yyyymm();
    let mut classes = Classes::new();
    if let Ok(mut canceled) = Canceled::scrape_into_iter_parse(&yyyymm).await {
        if canceled.len() > 10 {
            classes.canceled = canceled.drain(0..10).collect();
        } else {
            classes.canceled = canceled;
        }
    }
    if let Ok(mut moved) = Moved::scrape_into_iter_parse(&yyyymm).await {
        if moved.len() > 10 {
            classes.moved = moved.drain(0..10).collect();
        } else {
            classes.moved = moved;
        }
    }
    if let Ok(mut supplementary) = Supplementary::scrape_into_iter_parse(&yyyymm).await {
        if supplementary.len() > 10 {
            classes.supplementary = supplementary.drain(0..10).collect();
        } else {
            classes.supplementary = supplementary;
        }
    }
    classes.to_json()
}

async fn get_classes_canceled() -> impl Responder {
    let yyyymm = get_yyyymm();
    if let Ok(mut canceled) = Canceled::scrape_into_iter_parse(&yyyymm).await {
        if canceled.len() > 10 {
            return serde_json::to_string(&canceled.drain(0..10).collect::<Vec<Canceled>>());
        } else {
            return serde_json::to_string(&canceled);
        }
    }
    serde_json::to_string(&String::new())
}

async fn get_classes_moved() -> impl Responder {
    let yyyymm = get_yyyymm();
    if let Ok(mut moved) = Moved::scrape_into_iter_parse(&yyyymm).await {
        if moved.len() > 10 {
            return serde_json::to_string(&moved.drain(0..10).collect::<Vec<Moved>>());
        } else {
            return serde_json::to_string(&moved);
        }
    }
    serde_json::to_string(&String::new())
}

async fn get_classes_supplementary() -> impl Responder {
    let yyyymm = get_yyyymm();
    if let Ok(mut supplementary) = Supplementary::scrape_into_iter_parse(&yyyymm).await {
        if supplementary.len() > 10 {
            return serde_json::to_string(
                &supplementary.drain(0..10).collect::<Vec<Supplementary>>(),
            );
        } else {
            return serde_json::to_string(&supplementary);
        }
    }
    serde_json::to_string(&String::new())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(HttpResponse::Ok))
            .route("/api/classes/", web::get().to(get_classes))
            .route(
                "/api/classes/canceled/",
                web::get().to(get_classes_canceled),
            )
            .route("/api/classes/moved/", web::get().to(get_classes_moved))
            .route(
                "/api/classes/supplementary/",
                web::get().to(get_classes_supplementary),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
