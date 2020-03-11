use reqwest;
use scraper::{Html, Selector};

#[derive(Copy, Clone)]
pub enum Classes {
    Canceled,
    Moved,
    Supplymentaly,
}

#[derive(Debug)]
pub struct Scrape(pub Vec<String>);

impl Scrape {
    pub const fn new() -> Self {
        Scrape(Vec::new())
    }
    pub async fn scrape(&mut self, yyyymm: &str, classes: Classes) -> Result<(), ()> {
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
            .map(|c| c.inner_html())
            .collect::<Vec<_>>();
        let mut flag = String::new();
        let mut ju = Vec::new();
        let mut ho = Vec::new();
        let mut kyu = Vec::new();
        for content in contents {
            match content.as_str() {
                "授業変更" => flag = String::from("ju"),
                "補講" => flag = String::from("ho"),
                "休講" => flag = String::from("kyu"),
                _ => match flag.as_str() {
                    "ju" => ju.push(content.to_string()),
                    "ho" => ho.push(content.to_string()),
                    "kyu" => kyu.push(content.to_string()),
                    _ => return Err(()),
                },
            };
        }
        match classes {
            Classes::Canceled => {
                if kyu.is_empty() {
                    return Err(());
                }
                self.0 = kyu
            }
            Classes::Moved => {
                if ju.is_empty() {
                    return Err(());
                }
                self.0 = ju
            }
            Classes::Supplymentaly => {
                if ho.is_empty() {
                    return Err(());
                }
                self.0 = ho
            }
        }
        Ok(())
    }
}
