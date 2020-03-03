use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json;
use unofficial_api::Response;

async fn get_schedule(req: HttpRequest) -> impl Responder {
    let mut resp = Response::new();
    resp.scrape(req.match_info().get("yyyymm").unwrap_or("202001"))
        .await
        .unwrap();
    let result = resp.parse().unwrap();
    let result_json = serde_json::to_string(&result).unwrap();
    format!("{:?}", result_json)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(HttpResponse::Ok))
            .route("/{yyyymm}", web::get().to(get_schedule))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
