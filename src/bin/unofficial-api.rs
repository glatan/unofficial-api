use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use chrono::prelude::*;
use serde_json;
use serde::Serialize;
use unofficial_api::{Canceled, Classes, Moved, Scrape, Supplementary};

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


async fn scrape(class: Classes) -> Vec<String> {
    let yyyymm = String::from("201912");
    // コンフェス期間中は授業がなく、当然休講情報等は掲載されないので、2019年12月のものを取るようにしてます
    // let yyyymm = get_jst_yyyymm();
    let scraper = {
        if let Ok(scraper) = Scrape::classes(&yyyymm, class).await {
            scraper
        } else {
            // その月に何もなければ空のJSONを返す
            Vec::new()
            // return format!("{:?}", serde_json::to_string(&String::new()).unwrap());
        }
    };
    scraper
}

#[derive(Debug, PartialEq, Serialize)]
struct All {
    canceled: Vec<Canceled>,
    moved: Vec<Moved>,
    supplementary: Vec<Supplementary>,
}
impl All {
    pub fn new() -> Self {
        Self {
            canceled: Vec::new(),
            moved: Vec::new(),
            supplementary: Vec::new(),
        }
    }
    pub fn to_json(&self) -> Result<String, ()> {
        if let Ok(json) = serde_json::to_string(&self) {
            return Ok(json);
        } else {
            return Err(());
        }
    }
}

async fn get_classes() -> impl Responder {
    let yyyymm = String::from("201912");
    let mut classes = All::new();
    for c in scrape(Classes::Canceled).await {
        if let Ok(canceled) = Canceled::parse(&yyyymm, &c) {
            if classes.canceled.len() < 10 {
                classes.canceled.push(canceled);
            } else {
                break;
            }
        }
    }
    for c in scrape(Classes::Moved).await {
        if let Ok(moved) = Moved::parse(&yyyymm, &c) {
            if classes.moved.len() < 10 {
                classes.moved.push(moved);
            } else {
                break;
            }
        }
    }
    for c in scrape(Classes::Supplementary).await {
        if let Ok(supplementary) = Supplementary::parse(&yyyymm, &c) {
            if classes.supplementary.len() < 10 {
                classes.supplementary.push(supplementary);
            } else {
                break;
            }
        }
    }
    format!("{:?}", classes.to_json().unwrap())
}

async fn get_classes_canceled() -> impl Responder {
    let yyyymm = String::from("201912");
    let mut resp = Vec::with_capacity(10);
    for c in scrape(Classes::Canceled).await {
        if let Ok(canceled) = Canceled::parse(&yyyymm, &c) {
            if resp.len() < 10 {
                resp.push(canceled.to_json().unwrap());
            } else {
                break;
            }
        }
    }
    format!("{:?}", resp)
}

async fn get_classes_moved() -> impl Responder {
    let yyyymm = String::from("201912");
    let mut resp = Vec::with_capacity(10);
    for c in scrape(Classes::Moved).await {
        if let Ok(moved) = Moved::parse(&yyyymm, &c) {
            if resp.len() < 10 {
                resp.push(moved.to_json().unwrap());
            } else {
                break;
            }
        }
    }
    format!("{:?}", resp)
}

async fn get_classes_supplementary() -> impl Responder {
    let yyyymm = String::from("201912");
    let mut resp = Vec::with_capacity(10);
    for c in scrape(Classes::Supplementary).await {
        if let Ok(supplementary) = Supplementary::parse(&yyyymm, &c) {
            if resp.len() < 10 {
                resp.push(supplementary.to_json().unwrap());
            } else {
                break;
            }
        }
    }
    format!("{:?}", resp)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(HttpResponse::Ok))
            .route(
                "/api/classes/",
                web::get().to(get_classes),
            )
            .route(
                "/api/classes/canceled/",
                web::get().to(get_classes_canceled),
            )
            .route(
                "/api/classes/moved/",
                web::get().to(get_classes_moved),
            )
            .route(
                "/api/classes/supplementary/",
                web::get().to(get_classes_supplementary),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
