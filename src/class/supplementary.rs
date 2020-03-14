use crate::class::{Class, ClassNumber};
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
    pub fn parse(yyyymm: &str, entry: &str) -> Result<Self, ()> {
        let mut supplementary = Supplementary::new();
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
            },
            class: Class {
                date: "2019-12-20".to_string(),
                periods: [7, 8].to_vec(),
                name: "情報リテラシー".to_string(),
                teacher: "竹谷".to_string(),
                note: "多目的ホールで実施".to_string(),
            },
        };
        let mut supplementary = Supplementary::new();
        supplementary.parse(sample_id, sample).unwrap();
        assert_eq!(supplementary, expected);
    }
}
