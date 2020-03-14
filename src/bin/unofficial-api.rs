use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use chrono::prelude::*;
use serde_json;
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

async fn get_classes(class_type: Classes) -> impl Responder {
    let mut resp = Vec::with_capacity(10);
    // コンフェス期間中は授業がなく、当然休講情報等は掲載されないので、2019年12月のものを取るようにしてます
    // let yyyymm = get_jst_yyyymm();
    let yyyymm = "201912".to_string();
    let scraper = {
        if let Ok(scraper) = Scrape::classes(&yyyymm, class_type).await {
            scraper
        } else {
            // その月に何もなければ空のJSONを返す
            return format!("{:?}", serde_json::to_string(&String::new()).unwrap());
        }
    };
    for c in scraper {
        match class_type {
            Classes::Canceled => {
                if let Ok(canceled) = Canceled::parse(&yyyymm, &c) {
                    if resp.len() < 10 {
                        resp.push(serde_json::to_string(&canceled).unwrap());
                    } else {
                        break;
                    }
                }
            }
            Classes::Moved => {
                if let Ok(moved) = Moved::parse(&yyyymm, &c) {
                    if resp.len() < 10 {
                        resp.push(serde_json::to_string(&moved).unwrap());
                    } else {
                        break;
                    }
                }
            }
            Classes::Supplementary => {
                if let Ok(supplementary) = Supplementary::parse(&yyyymm, &c) {
                    if resp.len() < 10 {
                        resp.push(serde_json::to_string(&supplementary).unwrap());
                    } else {
                        break;
                    }
                }
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
                "/api/classes/canceled/",
                web::get().to(|| get_classes(Classes::Canceled)),
            )
            .route(
                "/api/classes/moved/",
                web::get().to(|| get_classes(Classes::Moved)),
            )
            .route(
                "/api/classes/supplementary/",
                web::get().to(|| get_classes(Classes::Supplementary)),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
