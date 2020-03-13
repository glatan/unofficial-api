use crate::parse::Parse;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Canceled {
    id: String,
    #[serde(rename(serialize = "classNumber"))]
    class_number: String,
    #[serde(flatten)]
    class_info: Parse,
}

impl Canceled {
    pub const fn new() -> Self {
        Canceled {
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
    use crate::classes::canceled::Canceled;
    use crate::parse::Parse;
    #[test]
    fn parse_class_number() {
        let sample_id = "201912";
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let mut canceled = Canceled::new();
        canceled.parse(sample_id, sample).unwrap();
        assert_eq!(
            canceled.class_number,
            "4-S（数学・物理科学プログラム）".to_string()
        );
    }
    #[test]
    fn parse_all() {
        let sample_id = "201912";
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let sample_result = Canceled {
            id: "2019-12".to_string(),
            class_number: "4-S（数学・物理科学プログラム）".to_string(),
            class_info: Parse {
                date: "2019-12-05".to_string(),
                periods: [3, 4].to_vec(),
                class_name: "集合と位相".to_string(),
                teacher: "吉田".to_string(),
                note: "補講実施予定".to_string(),
            },
        };
        let mut canceled = Canceled::new();
        canceled.parse(sample_id, sample).unwrap();
        assert_eq!(canceled, sample_result);
    }
}
