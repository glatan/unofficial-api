use regex::Regex;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Parse {
    pub(crate) date: String,
    pub(crate) periods: Vec<u8>,
    #[serde(rename = "className")]
    pub(crate) class_name: String,
    pub(crate) teacher: String,
    pub(crate) note: String,
}

impl Parse {
    pub const fn new() -> Self {
        Parse {
            date: String::new(),
            periods: Vec::new(),
            class_name: String::new(),
            teacher: String::new(),
            note: String::new(),
        }
    }
    pub fn class_info(mut entry: &str) -> Result<Parse, ()> {
        // DO NOT CHANGE THE PARSE ORDER.
        entry = entry.trim_start().trim_end();
        let mut class_info: Parse = Parse::new();
        // note
        let note_regex = Regex::new(r"(【.+】)+$").unwrap();
        if let Some(note_index) = note_regex.find(entry) {
            let (other, note) = entry.split_at(note_index.start());
            entry = other.trim_end();
            let trimer: &[_] = &['【', '】'];
            // <note>, <note>
            class_info.note = note.trim_matches(trimer).replace("】【", ",");
        }
        // date
        let date_regex = Regex::new(r"^(\d+月\d+日)\(.+\)").unwrap();
        if let Some(date_index) = date_regex.find(entry) {
            let (date, other) = entry.split_at(date_index.end());
            entry = other.trim_start();
            // Convert MM月DD日 to MM-DD
            let date_format_regex = Regex::new(r"(?P<month>\d+)月(?P<day>\d+)日").unwrap();
            let date_format_regex_result = date_format_regex.captures(date).unwrap();
            let month = {
                let month = date_format_regex_result.name("month").unwrap().as_str();
                match month.len() {
                    // zero padding
                    1 => format!("0{}", month),
                    _ => month.to_string(),
                }
            };
            let day = {
                let day = date_format_regex_result.name("day").unwrap().as_str();
                match day.len() {
                    // zero padding
                    1 => format!("0{}", day),
                    _ => day.to_string(),
                }
            };
            class_info.date = format!("{}-{}", month, day);
        }
        // Trim class_number
        let class_number_regex = Regex::new(r"^((\w-\w)|(専.+))\S*").unwrap();
        if let Some(class_number_index) = class_number_regex.find(entry) {
            let (_, other) = entry.split_at(class_number_index.end());
            entry = other.trim_start();
        }
        // Trim program
        let program_regex = Regex::new(r"^（.+プログラム）").unwrap();
        if let Some(program_index) = program_regex.find(entry) {
            let (_, other) = entry.split_at(program_index.end());
            entry = other.trim_start();
        }
        // periods
        let periods_regex = Regex::new(r"^\[(\d|\d・\d|\d～\d)限\]\s").unwrap();
        if let Some(periods_index) = periods_regex.find(entry) {
            let (periods, other) = entry.split_at(periods_index.end());
            entry = other.trim_start();
            // convert <periods> to [period, period]: Vec<u8>
            // period_range: 要素は1つか2つ。2個あってその差が1以上なら2コマ以上
            let mut period_range: Vec<u8> = Vec::new();
            for c in periods.chars() {
                // n限目のnが2桁になることは想定していない
                if let Ok(number) = c.to_string().parse::<u8>() {
                    period_range.push(number);
                }
            }
            for i in period_range[0]..=period_range[1] {
                class_info.periods.push(i)
            }
        };
        // teacher
        let teacher_regex = Regex::new(r"（.+）$").unwrap();
        if let Some(teacher_index) = teacher_regex.find(entry) {
            let (other, teacher) = entry.split_at(teacher_index.start());
            entry = other.trim_end();
            let trim_matches: &[_] = &['（', '）'];
            class_info.teacher = teacher.trim_matches(trim_matches).to_string();
        }
        // class_name
        if !entry.is_empty() {
            class_info.class_name = entry.to_string();
        }
        // return Err(entry.to_string());
        Ok(class_info)
    }
    pub fn class_number(entry: &str) -> Result<String, ()> {
        let mut class_number = String::new();
        // e.g. 4-S
        let class_number_regex = Regex::new(r"(?P<class_number>((\w-\w)|(専.+))\S*)").unwrap();
        if let Some(c) = class_number_regex.captures(entry) {
            class_number = c.name("class_number").unwrap().as_str().to_string();
        }
        // e.g. （数学・物理科学プログラム）
        // let program_regex = Regex::new(r"(?P<program>（.+プログラム）)").unwrap();
        // if let Some(c) = program_regex.captures(entry) {
        //     class_number += c.name("program").unwrap().as_str();
        // }
        Ok(class_number)
    }
}

#[cfg(test)]
mod test {
    use crate::parse::Parse;
    #[test]
    fn date_parse() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let result = Parse::class_info(sample).unwrap().date;
        assert_eq!(result, "12-05".to_string());
    }
    #[test]
    fn period_parse() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let result = Parse::class_info(sample).unwrap().periods;
        assert_eq!(result, [3, 4]);
    }
    #[test]
    fn note_parse() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let result = Parse::class_info(sample).unwrap().note;
        assert_eq!(result, "補講実施予定".to_string());
    }
    #[test]
    fn teacher_test() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let result = Parse::class_info(sample).unwrap().teacher;
        assert_eq!(result, "吉田".to_string());
    }
    #[test]
    fn class_name_test() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let result = Parse::class_info(sample).unwrap().class_name;
        assert_eq!(result, "集合と位相".to_string());
    }
}
