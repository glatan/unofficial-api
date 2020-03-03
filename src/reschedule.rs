use crate::error::{Error, Kind};

use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Reschedule {
    ju: Vec<(Body, Body)>,
    ho: Vec<Body>,
    kyu: Vec<Body>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Body {
    date: String,
    class_num: String,
    program: String,
    pub class: Vec<Class>,
    option: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Class {
    count: String,
    name: String,
    teacher: String,
}

#[derive(Debug)]
pub struct Response {
    ju: Vec<String>,  // 授業変更
    ho: Vec<String>,  // 補講
    kyu: Vec<String>, // 休講
}

impl Response {
    pub fn new() -> Response {
        Response {
            ju: Vec::new(),
            ho: Vec::new(),
            kyu: Vec::new(),
        }
    }
    pub async fn scrape(&mut self, yyyymm: &str) -> Result<(), Error> {
        let url = format!(
            "http://www.tsuyama-ct.ac.jp/oshiraseVer4/renraku/renraku{}.html",
            yyyymm
        );
        let resp = {
            let resp = reqwest::get(&url).await;
            match resp {
                Ok(resp) => resp,
                Err(e) => {
                    return Err(Error {
                        kind: Kind::HTTP,
                        cause: format!("{}", e),
                    })
                }
            }
        };
        let body = {
            let body = resp.text().await;
            match body {
                Ok(body) => body,
                Err(e) => {
                    return Err(Error {
                        kind: Kind::HTML,
                        cause: format!("{}", e),
                    })
                }
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
        for content in contents.iter().take(contents.len() - 1) {
            match content.as_str() {
                "授業変更" => flag = String::from("ju"),
                "補講" => flag = String::from("ho"),
                "休講" => flag = String::from("kyu"),
                _ => match flag.as_str() {
                    "ju" => ju.push(content),
                    "ho" => ho.push(content),
                    "kyu" => kyu.push(content),
                    _ => {
                        return Err(Error {
                            kind: Kind::HTML,
                            cause: "Cannot find \"授業変更\" or \"補講\" or \"休講\"".to_string(),
                        })
                    }
                },
            };
        }
        self.ju = ju.iter().map(|&c| c.to_string()).collect::<Vec<String>>();
        self.ho = ho.iter().map(|&c| c.to_string()).collect::<Vec<String>>();
        self.kyu = kyu.iter().map(|&c| c.to_string()).collect::<Vec<String>>();
        Ok(())
    }
    pub fn parse(&mut self) -> Result<Reschedule, Error> {
        // <date> <class_num> <program> <class_count> |loop <class + teacher>|
        // <11月12日(火)> <3-S> <[3限] 学修時間、[4限]LHR → [3・4限]> <熱力学概論（佐伯）【入替】>
        // <\d+月\d+日> <(\w-\w)|(専.+\d)> <\[(\d|\d・\d)限\]>(if '、' loop) <.+（.+）【.+】>|<.+\s【.+】>
        let core_regex = Regex::new(r"(?P<date>\d+月\d+日)\(.+\)\s(?P<class_num>(\w-\w)|(専.+)\S+)\s(\((?P<program>.+)\)|\[)").unwrap();
        let mut ju: Vec<(Body, Body)> = Default::default();
        for content in &self.ju {
            let mut before: Body = Default::default();
            for caps in core_regex.captures_iter(content) {
                before.date = match caps.name("date") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                before.class_num = match caps.name("class_num") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                before.program = match caps.name("program") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
            }
            // classes[0](矢印の左側): "XX月XX日(X) X-X [X・X限] X（XX）"
            // classes[1](矢印の右側): "XX月XX日(X) X-X [X・X限] X（XX）"
            let classes: Vec<&str> = content
                .split(|c| c == '→' || c == '←')
                .map(|c| c.trim())
                .collect();
            let classes_regex =
                Regex::new(r"\[(?P<count>(\d|\d・\d))限\]\s(?P<name>[^（）]+)（(?P<teacher>.+)）$")
                    .unwrap();
            for caps in classes_regex.captures_iter(classes[0]) {
                let count = match caps.name("count") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                let name = match caps.name("name") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                let teacher = match caps.name("teacher") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                before.class.push(Class {
                    count: count,
                    name: name,
                    teacher: teacher,
                });
            }
            // println!("{:?}", before);
            let mut after: Body = Default::default();
            // let after_regex = Regex::new(r"(?P<date>\d+月\d+日).*\[(?P<count>(\d|\d・\d)限)\](?P<class>.+).*（(?P<teacher>.+)）.*(【(?P<option>.+)】)").unwrap();
            // for caps in core_regex.captures_iter(classes[1]) {
            //     after.date = match caps.name("date") {
            //         Some(t) => t.as_str().to_string(),
            //         None => before.date.clone()
            //     };
            //     after.class_num = match caps.name("class_num") {
            //         Some(t) => t.as_str().to_string(),
            //         None => before.class_num.clone()
            //     };
            //     after.program = match caps.name("program") {
            //         Some(t) => t.as_str().to_string(),
            //         None => before.program.clone()
            //     };
            // }
            let after_classes_regex =
                Regex::new(r"(?P<name>[^（）]+)（(?P<teacher>.+)）【(?P<option>.+)】").unwrap();
            for caps in after_classes_regex.captures_iter(classes[1]) {
                let count = match caps.name("count") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                let name = match caps.name("name") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                let teacher = match caps.name("teacher") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                after.class.push(Class {
                    count: count,
                    name: name,
                    teacher: teacher,
                });
                after.option = match caps.name("option") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
            }
            if after.date == String::new() {
                after.date = before.date.clone();
            }
            if after.class_num == String::new() {
                after.class_num = before.class_num.clone();
            }
            if after.program == String::new() {
                after.program = before.program.clone();
            }
            ju.push((before, after));
        }
        let mut ho: Vec<Body> = Default::default();
        for content in &self.ho {
            let mut before: Body = Default::default();
            for caps in core_regex.captures_iter(content) {
                before.date = match caps.name("date") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                before.class_num = match caps.name("class_num") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                before.program = match caps.name("program") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
            }
            // classes[0](矢印の左側): "XX月XX日(X) X-X [X・X限] X（XX）"
            // classes[1](矢印の右側): "XX月XX日(X) X-X [X・X限] X（XX）"
            let classes: Vec<&str> = content
                .split(|c| c == '→' || c == '←')
                .map(|c| c.trim())
                .collect();
            let classes_regex =
                Regex::new(r"\[(?P<count>(\d|\d・\d))限\]\s(?P<name>[^（）]+)（(?P<teacher>.+)）$")
                    .unwrap();
            for caps in classes_regex.captures_iter(classes[0]) {
                let count = match caps.name("count") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                let name = match caps.name("name") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                let teacher = match caps.name("teacher") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                before.class.push(Class {
                    count: count,
                    name: name,
                    teacher: teacher,
                });
            }
            ho.push(before);
        }
        let mut kyu: Vec<Body> = Default::default();
        for content in &self.kyu {
            let mut before: Body = Default::default();
            for caps in core_regex.captures_iter(content) {
                before.date = match caps.name("date") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                before.class_num = match caps.name("class_num") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                before.program = match caps.name("program") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
            }
            // classes[0](矢印の左側): "XX月XX日(X) X-X [X・X限] X（XX）"
            // classes[1](矢印の右側): "XX月XX日(X) X-X [X・X限] X（XX）"
            let classes: Vec<&str> = content
                .split(|c| c == '→' || c == '←')
                .map(|c| c.trim())
                .collect();
            let classes_regex =
                Regex::new(r"\[(?P<count>(\d|\d・\d))限\]\s(?P<name>[^（）]+)（(?P<teacher>.+)）$")
                    .unwrap();
            for caps in classes_regex.captures_iter(classes[0]) {
                let count = match caps.name("count") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                let name = match caps.name("name") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                let teacher = match caps.name("teacher") {
                    Some(t) => t.as_str().to_string(),
                    None => String::new(),
                };
                before.class.push(Class {
                    count: count,
                    name: name,
                    teacher: teacher,
                });
            }
            kyu.push(before);
        }
        Ok(Reschedule {
            ju: ju,
            ho: ho,
            kyu: kyu,
        })
    }
}

// impl Reschedule {

// }
