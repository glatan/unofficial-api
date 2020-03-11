use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use serde_json;
use unofficial_api::{Canceled, Classes, Moved, Scrape, Supplymentaly};

fn get_jst_yyyymm() -> String {
    let dt = FixedOffset::east(9 * 3600);
    let jst_now = Utc::now().with_timezone(&dt);
    jst_now.format("%Y%m").to_string()
}

fn minus_one_month(yyyymm: &str) -> String {
    let (yyyy, mm) = yyyymm.split_at(4);
    let (subtracted_yyyy, subtracted_mm);
    if mm == "01" {
        subtracted_yyyy = (yyyy.parse::<u16>().unwrap() - 1).to_string();
        subtracted_mm = "12".to_string();
    } else {
        subtracted_yyyy = yyyy.to_string();
        subtracted_mm = {
            let m = (mm.parse::<u16>().unwrap() - 1).to_string();
            match m.len() {
                1 => "0".to_string() + &m,
                2 => m,
                _ => panic!("Invalid mm"),
            }
        };
    }
    subtracted_yyyy + &subtracted_mm
}

async fn get_classes(class_type: Classes) -> impl Responder {
    let mut resp = Vec::with_capacity(10);
    let mut yyyymm = get_jst_yyyymm();
    // 10件取得
    while resp.len() < 10 {
        let mut scrape_result = Vec::with_capacity(10);
        while scrape_result.len() < 10 {
            let mut scraper = Scrape::new();
            // "エラーが帰ってきたら一ヶ月前ので試してみる"を繰り返す
            while let Err(_) = scraper.scrape(&yyyymm, class_type).await {
                yyyymm = minus_one_month(&yyyymm);
            }
            if scraper.0.len() < 10 {
                yyyymm = minus_one_month(&yyyymm);
            }
            scrape_result.append(&mut scraper.0);
        }
        for c in scrape_result {
            println!("{:?}", c);
            match class_type {
                Classes::Canceled => {
                    let mut canceled = Canceled::new();
                    if canceled.parse(&yyyymm, &c).is_ok() {
                        if resp.len() < 10 {
                            resp.push(serde_json::to_string(&canceled).unwrap());
                        } else {
                            break;
                        }
                    }
                }
                Classes::Moved => {
                    let mut moved = Moved::new();
                    if moved.parse(&yyyymm, &c).is_ok() {
                        if resp.len() < 10 {
                            resp.push(serde_json::to_string(&moved).unwrap());
                        } else {
                            break;
                        }
                    }
                }
                Classes::Supplymentaly => {
                    let mut supplymentaly = Supplymentaly::new();
                    if supplymentaly.parse(&yyyymm, &c).is_ok() {
                        if resp.len() < 10 {
                            resp.push(serde_json::to_string(&supplymentaly).unwrap());
                        } else {
                            break;
                        }
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
                "/api/classes/supplymentaly/",
                web::get().to(|| get_classes(Classes::Supplymentaly)),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
