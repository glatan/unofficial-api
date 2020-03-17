use reqwest;
use scraper::{Html, Selector};

pub struct HtmlGetter;
impl HtmlGetter {
    pub async fn get_renraku(yyyymm: &str) -> Result<Vec<String>, ()> {
        let url = format!(
            "http://www.tsuyama-ct.ac.jp/oshiraseVer4/renraku/renraku{}.html",
            yyyymm
        );
        let resp = {
            if let Ok(resp) = reqwest::get(&url).await {
                resp
            } else {
                return Err(());
            }
        };
        let body = {
            if let Ok(body) = resp.text().await {
                body
            } else {
                return Err(());
            }
        };
        let document = Html::parse_document(&body);
        let selector = Selector::parse("div#contents h4, div#contents p").unwrap();
        let contents = document
            .select(&selector)
            .map(|c| c.html())
            .collect::<Vec<_>>();
        Ok(contents)
    }
}
