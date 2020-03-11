use crate::parse::Parse;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Moved {
    pub(crate) id: String,
    #[serde(rename(serialize = "classNumber"))]
    pub(crate) class_number: String,
    pub(crate) before: Parse,
    pub(crate) after: Parse,
}

impl Moved {
    pub const fn new() -> Self {
        Moved {
            id: String::new(),
            class_number: String::new(),
            before: Parse::new(),
            after: Parse::new(),
        }
    }
    pub fn parse(&mut self, yyyymm: &str, entry: &str) -> Result<(), ()> {
        // Convert YYYY-MM to YYYY-MM
        let (year, month) = yyyymm.split_at(4);
        let id = String::from(year) + "-" + month;
        self.id = id;
        let (mut before, mut after) = (String::new(), String::new());
        if let Some(n) = entry.find('→') {
            let (b, a) = entry.split_at(n);
            before = b.trim().to_string();
            after = a.trim().trim_matches('→').trim().to_string();
        } else if let Some(n) = entry.find('←') {
            let (a, b) = entry.split_at(n);
            before = b.trim().to_string();
            after = a.trim().trim_matches('←').trim().to_string();
        }
        self.before = Parse::class_info(&before).unwrap();
        self.after = Parse::class_info(&after).unwrap();
        self.class_number = Parse::class_number(&entry).unwrap();
        // Parse::parse(&mut self.entry).unwrap().date : MM-DD
        // Convert it to YYYY-MM-DD
        let (year, _) = self.id.split_at(4);
        self.before.date = format!("{}-{}", year, self.before.date);
        if self.after.date.is_empty() {
            self.after.date = self.before.date.clone();
        } else {
            self.after.date = format!("{}-{}", year, self.after.date);
        }
        self.after.periods = self.before.periods.clone();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::classes::moved::Moved;
    use crate::parse::Parse;
    #[test]
    fn parse_class_number() {
        let sample_id = "201912";
        let sample = "12月3日(火) 3-C [1・2限] ディジタル応用（川波）→ 国語III（杉山）【入替】";
        let mut moved = Moved::new();
        moved.parse(sample_id, sample).unwrap();
        assert_eq!(moved.class_number, "3-C".to_string());
    }
    #[test]
    fn parse_all() {
        let sample_id = "201912";
        let sample = "12月3日(火) 3-C [1・2限] ディジタル応用（川波）→ 国語III（杉山）【入替】";
        let sample_result = Moved {
            id: "2019-12".to_string(),
            class_number: "3-C".to_string(),
            before: Parse {
                date: "2019-12-03".to_string(),
                periods: [1, 2].to_vec(),
                class_name: "ディジタル応用".to_string(),
                teacher: "川波".to_string(),
                note: "".to_string(),
            },
            after: Parse {
                date: "2019-12-03".to_string(),
                periods: [1, 2].to_vec(),
                class_name: "国語III".to_string(),
                teacher: "杉山".to_string(),
                note: "入替".to_string(),
            },
        };
        let mut moved = Moved::new();
        moved.parse(sample_id, sample).unwrap();
        assert_eq!(moved, sample_result);
    }
}
