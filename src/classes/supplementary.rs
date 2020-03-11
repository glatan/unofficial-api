use crate::parse::Parse;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Supplementary {
    pub(crate) id: String,
    #[serde(rename(serialize = "classNumber"))]
    pub(crate) class_number: String,
    #[serde(flatten)]
    pub(crate) class_info: Parse,
}

impl Supplementary {
    pub const fn new() -> Self {
        Supplementary {
            id: String::new(),
            class_number: String::new(),
            class_info: Parse::new(),
        }
    }
    pub fn parse(&mut self, yyyymm: &str, entry: &str) -> Result<(), ()> {
        // Convert YYYY-MM to YYYY-MM
        let (year, month) = yyyymm.split_at(4);
        let id = String::from(year) + "-" + month;
        self.id = id;
        self.class_info = Parse::class_info(&entry).unwrap();
        self.class_number = Parse::class_number(&entry).unwrap();
        // Parse::parse(&mut self.entry).unwrap().date : MM-DD
        // Convert it to YYYY-MM-DD
        let (year, _) = self.id.split_at(4);
        self.class_info.date = format!("{}-{}", year, self.class_info.date);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::classes::supplementary::Supplementary;
    use crate::parse::Parse;
    #[test]
    fn parse_class_number() {
        let sample_id = "201912";
        let sample = "12月20日(金) 1-3 [7・8限] 情報リテラシー（竹谷）【多目的ホールで実施】";
        let mut supplementary = Supplementary::new();
        supplementary.parse(sample_id, sample).unwrap();
        assert_eq!(supplementary.class_number, "1-3".to_string());
    }
    #[test]
    fn parse_all() {
        let sample_id = "201912";
        let sample = "12月20日(金) 1-3 [7・8限] 情報リテラシー（竹谷）【多目的ホールで実施】";
        let sample_result = Supplementary {
            id: "2019-12".to_string(),
            class_number: "1-3".to_string(),
            class_info: Parse {
                date: "2019-12-20".to_string(),
                periods: [7, 8].to_vec(),
                class_name: "情報リテラシー".to_string(),
                teacher: "竹谷".to_string(),
                note: "多目的ホールで実施".to_string(),
            },
        };
        let mut supplementary = Supplementary::new();
        supplementary.parse(sample_id, sample).unwrap();
        assert_eq!(supplementary, sample_result);
    }
}
