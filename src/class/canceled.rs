use crate::class::{Class, ClassNumber};
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
        Canceled {
            id: String::new(),
            class_number: ClassNumber::new(),
            class: Class::new(),
        }
    }
    pub fn parse(&mut self, yyyymm: &str, entry: &str) -> Result<(), ()> {
        // Convert YYYY-MM to YYYY-MM
        let (year, month) = yyyymm.split_at(4);
        let id = String::from(year) + "-" + month;
        self.id = id;
        self.class = Class::parse(&entry).unwrap();
        self.class_number = ClassNumber::parse(&entry).unwrap();
        // Class::parse(&mut self.entry).unwrap().date : MM-DD
        // Convert it to YYYY-MM-DD
        let (year, _) = self.id.split_at(4);
        self.class.date = format!("{}-{}", year, self.class.date);
        Ok(())
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
            },
            class: Class {
                date: "2019-12-05".to_string(),
                periods: [3, 4].to_vec(),
                name: "集合と位相".to_string(),
                teacher: "吉田".to_string(),
                note: "補講実施予定".to_string(),
            },
        };
        let mut canceled = Canceled::new();
        canceled.parse(sample_id, sample).unwrap();
        assert_eq!(canceled, expected);
    }
}
