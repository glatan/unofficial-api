use crate::class::{Class, ClassNumber, HtmlGetter};

use regex::Regex;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Moved {
    id: String,
    #[serde(rename(serialize = "classNumber"))]
    class_number: ClassNumber,
    before: Class,
    after: Class,
}

impl Moved {
    pub const fn new() -> Self {
        Self {
            id: String::new(),
            class_number: ClassNumber::new(),
            before: Class::new(),
            after: Class::new(),
        }
    }
    pub async fn scrape(yyyymm: &str) -> Result<Vec<String>, ()> {
        let contents = HtmlGetter::get_renraku(yyyymm).await?;
        let mut found_moved_parent = false;
        let mut moved = Vec::new();
        let trim_tag = Regex::new(r"<p>(?P<inner>.+)</p>").unwrap();
        for content in contents {
            if found_moved_parent && content.starts_with("<p>") {
                if let Some(inner) = trim_tag.captures(&content) {
                    moved.push(inner.name("inner").unwrap().as_str().to_string());
                }
            } else if content == "<h4>授業変更</h4>" {
                found_moved_parent = true;
            } else if content.starts_with("<h4>") {
                found_moved_parent = false;
            }
        }
        Ok(moved)
    }
    pub fn parse(yyyymm: &str, entry: &str) -> Result<Self, ()> {
        let mut moved = Self::new();
        // Convert YYYY-MM to YYYY-MM
        let (year, month) = yyyymm.split_at(4);
        let id = String::from(year) + "-" + month;
        moved.id = id;
        let (before, after) = {
            let (before, after);
            if let Some(n) = entry.find('→') {
                let (b, a) = entry.split_at(n);
                before = b.to_string();
                after = a.trim_start_matches('→').to_string();
            } else if let Some(n) = entry.find('←') {
                let (b, a) = entry.split_at(n);
                before = b.to_string();
                after = a.trim_start_matches('←').to_string();
            } else {
                return Err(());
            }
            (before, after)
        };
        moved.before = Class::parse(&before)?;
        moved.after = Class::parse(&after)?;
        moved.class_number = ClassNumber::parse(&entry)?;
        // Class::parse(&mut moved.entry).unwrap().date : MM-DD
        // Convert it to YYYY-MM-DD
        let (year, _) = moved.id.split_at(4);
        moved.before.date = format!("{}-{}", year, moved.before.date);
        if moved.after.date.is_empty() {
            moved.after.date = moved.before.date.clone();
        } else {
            moved.after.date = format!("{}-{}", year, moved.after.date);
        }
        moved.after.periods = moved.before.periods.clone();
        Ok(moved)
    }
    pub async fn scrape_into_iter_parse(yyyymm: &str) -> Result<Vec<Self>, ()> {
        let scrape_result = Self::scrape(yyyymm).await?;
        let mut moved = Vec::new();
        for entry in scrape_result {
            moved.push(Self::parse(yyyymm, &entry)?);
        }
        Ok(moved)
    }
}

#[cfg(test)]
mod test {
    use crate::class::moved::Moved;
    use crate::class::{Class, ClassNumber};
    #[test]
    fn parse_all() {
        let sample_id = "201912";
        let sample = "12月3日(火) 3-C [1・2限] ディジタル応用（川波）→ 国語III（杉山）【入替】";
        let expected = Moved {
            id: "2019-12".to_string(),
            class_number: ClassNumber {
                grade: 3,
                program: "C".to_string(),
                former_class: false,
                regular_course: true,
                note: String::new(),
            },
            before: Class {
                date: "2019-12-03".to_string(),
                periods: [1, 2].to_vec(),
                name: "ディジタル応用".to_string(),
                teacher: "川波".to_string(),
                note: "".to_string(),
            },
            after: Class {
                date: "2019-12-03".to_string(),
                periods: [1, 2].to_vec(),
                name: "国語III".to_string(),
                teacher: "杉山".to_string(),
                note: "入替".to_string(),
            },
        };
        let moved = Moved::parse(sample_id, sample).unwrap();
        assert_eq!(moved, expected);
    }
}
