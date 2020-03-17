use crate::class::{Class, ClassNumber, HtmlGetter};

use regex::Regex;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Supplementary {
    id: String,
    #[serde(rename(serialize = "classNumber"))]
    class_number: ClassNumber,
    #[serde(flatten)]
    class: Class,
}

impl Supplementary {
    pub const fn new() -> Self {
        Self {
            id: String::new(),
            class_number: ClassNumber::new(),
            class: Class::new(),
        }
    }
    pub async fn scrape(yyyymm: &str) -> Result<Vec<String>, ()> {
        let contents = HtmlGetter::get_renraku(yyyymm).await?;
        let mut found_supplementary_parent = false;
        let mut supplementary = Vec::new();
        let trim_tag = Regex::new(r"<p>(?P<inner>.+)</p>").unwrap();
        for content in contents {
            if found_supplementary_parent && content.starts_with("<p>") {
                if let Some(inner) = trim_tag.captures(&content) {
                    supplementary.push(inner.name("inner").unwrap().as_str().to_string());
                }
            } else if content == "<h4>補講</h4>" {
                found_supplementary_parent = true;
            } else if content.starts_with("<h4>") {
                found_supplementary_parent = false;
            }
        }
        Ok(supplementary)
    }
    pub fn parse(yyyymm: &str, entry: &str) -> Result<Self, ()> {
        let mut supplementary = Self::new();
        // Convert YYYY-MM to YYYY-MM
        let (year, month) = yyyymm.split_at(4);
        let id = String::from(year) + "-" + month;
        supplementary.id = id;
        supplementary.class = Class::parse(&entry)?;
        supplementary.class_number = ClassNumber::parse(&entry)?;
        // Class::parse(&mut supplementary.entry).unwrap().date : MM-DD
        // Convert it to YYYY-MM-DD
        let (year, _) = supplementary.id.split_at(4);
        supplementary.class.date = format!("{}-{}", year, supplementary.class.date);
        Ok(supplementary)
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
    use crate::class::supplementary::Supplementary;
    use crate::class::{Class, ClassNumber};
    #[test]
    fn parse_all() {
        let sample_id = "201912";
        let sample = "12月20日(金) 1-3 [7・8限] 情報リテラシー（竹谷）【多目的ホールで実施】";
        let expected = Supplementary {
            id: "2019-12".to_string(),
            class_number: ClassNumber {
                grade: 1,
                program: "3".to_string(),
                former_class: false,
                regular_course: true,
                note: String::new(),
            },
            class: Class {
                date: "2019-12-20".to_string(),
                periods: [7, 8].to_vec(),
                name: "情報リテラシー".to_string(),
                teacher: "竹谷".to_string(),
                note: "多目的ホールで実施".to_string(),
            },
        };
        let supplementary = Supplementary::parse(sample_id, sample).unwrap();
        assert_eq!(supplementary, expected);
    }
}
