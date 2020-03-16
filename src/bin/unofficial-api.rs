use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Result};
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

async fn get_classes() -> Result<HttpResponse, HttpResponse> {
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
    Ok(HttpResponse::Ok().json(&classes))
}

async fn get_classes_canceled() -> Result<HttpResponse, HttpResponse> {
    let yyyymm = get_yyyymm();
    if let Ok(mut canceled) = Canceled::scrape_into_iter_parse(&yyyymm).await {
        if canceled.len() > 10 {
            return Ok(HttpResponse::Ok().json(&canceled.drain(0..10).collect::<Vec<Canceled>>()));
        } else {
            return Ok(HttpResponse::Ok().json(&canceled));
        }
    }
    Err(HttpResponse::InternalServerError().body("Failed to get canceled."))
}

async fn get_classes_moved() -> Result<HttpResponse, HttpResponse> {
    let yyyymm = get_yyyymm();
    if let Ok(mut moved) = Moved::scrape_into_iter_parse(&yyyymm).await {
        if moved.len() > 10 {
            return Ok(HttpResponse::Ok().json(&moved.drain(0..10).collect::<Vec<Moved>>()));
        } else {
            return Ok(HttpResponse::Ok().json(&moved));
        }
    }
    Err(HttpResponse::InternalServerError().body("Failed to get moved."))
}

async fn get_classes_supplementary() -> Result<HttpResponse, HttpResponse> {
    let yyyymm = get_yyyymm();
    if let Ok(mut supplementary) = Supplementary::scrape_into_iter_parse(&yyyymm).await {
        if supplementary.len() > 10 {
            return Ok(HttpResponse::Ok()
                .json(&supplementary.drain(0..10).collect::<Vec<Supplementary>>()));
        } else {
            return Ok(HttpResponse::Ok().json(&supplementary));
        }
    }
    Err(HttpResponse::InternalServerError().body("Failed to get supplementary."))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = {
        if let Ok(port) = env::var("PORT") {
            port
        } else {
            String::from("8000")
        }
    };
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
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
