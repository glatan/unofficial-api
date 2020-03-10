use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json;
use unofficial_api::{Canceled, Classes, Scrape};

async fn get_canceled(req: HttpRequest) -> impl Responder {
    let yyyymm = req.match_info().get("yyyymm").unwrap_or("202001");
    let mut resp = Scrape::new();
    resp.scrape(yyyymm, Classes::Canceled).await.unwrap();
    let mut result: Vec<Canceled> = Vec::new();
    for c in resp.0 {
        let mut canceled = Canceled::new();
        if canceled.parse(yyyymm, &c).is_ok() {
            result.push(canceled);
        }
    }
    let result_json = serde_json::to_string(&result).unwrap();
    format!("{:?}", result_json)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(HttpResponse::Ok))
            .route("/{yyyymm}", web::get().to(get_canceled))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
