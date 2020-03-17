use crate::class::{Class, ClassNumber, HtmlGetter};

use regex::Regex;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Canceled {
    id: String,
    #[serde(rename(serialize = "classNumber"))]
    class_number: ClassNumber,
    #[serde(flatten)]
    class: Class,
}

impl Canceled {
    pub const fn new() -> Self {
        Self {
            id: String::new(),
            class_number: ClassNumber::new(),
            class: Class::new(),
        }
    }
    pub async fn scrape(yyyymm: &str) -> Result<Vec<String>, ()> {
        let contents = HtmlGetter::get_renraku(yyyymm).await?;
        let mut found_canceled_parent = false;
        let mut canceled = Vec::new();
        let trim_tag = Regex::new(r"<p>(?P<inner>.+)</p>").unwrap();
        for content in contents {
            if found_canceled_parent && content.starts_with("<p>") {
                if let Some(inner) = trim_tag.captures(&content) {
                    canceled.push(inner.name("inner").unwrap().as_str().to_string());
                }
            } else if content == "<h4>休講</h4>" {
                found_canceled_parent = true;
            } else if content.starts_with("<h4>") {
                found_canceled_parent = false;
            }
        }
        Ok(canceled)
    }
    pub fn parse(yyyymm: &str, entry: &str) -> Result<Self, ()> {
        let mut canceled = Self::new();
        // Convert YYYY-MM to YYYY-MM
        let (year, month) = yyyymm.split_at(4);
        let id = String::from(year) + "-" + month;
        canceled.id = id;
        canceled.class = Class::parse(&entry)?;
        canceled.class_number = ClassNumber::parse(&entry)?;
        // Class::parse(&mut canceled.entry).unwrap().date : MM-DD
        // Convert it to YYYY-MM-DD
        let (year, _) = canceled.id.split_at(4);
        canceled.class.date = format!("{}-{}", year, canceled.class.date);
        Ok(canceled)
    }
    pub async fn scrape_into_iter_parse(yyyymm: &str) -> Result<Vec<Self>, ()> {
        let parse_result = Self::scrape(yyyymm).await?;
        let mut result = Vec::new();
        for entry in parse_result {
            result.push(Self::parse(yyyymm, &entry)?)
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::class::canceled::Canceled;
    use crate::class::{Class, ClassNumber};
    #[test]
    fn parse_all() {
        let sample_id = "201912";
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let expected = Canceled {
            id: "2019-12".to_string(),
            class_number: ClassNumber {
                grade: 4,
                program: "S".to_string(),
                former_class: false,
                regular_course: true,
                note: "数学・物理科学プログラム".to_string(),
            },
            class: Class {
                date: "2019-12-05".to_string(),
                periods: [3, 4].to_vec(),
                name: "集合と位相".to_string(),
                teacher: "吉田".to_string(),
                note: "補講実施予定".to_string(),
            },
        };
        let canceled = Canceled::parse(sample_id, sample).unwrap();
        assert_eq!(canceled, expected);
    }
}
