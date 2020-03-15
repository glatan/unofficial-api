mod canceled;
mod moved;
mod supplementary;

pub use canceled::Canceled;
pub use moved::Moved;
pub use supplementary::Supplementary;

use regex::Regex;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Class {
    pub(crate) date: String,
    pub(crate) periods: Vec<u8>,
    #[serde(rename = "className")]
    pub(crate) name: String,
    pub(crate) teacher: String,
    pub(crate) note: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ClassNumber {
    pub(crate) grade: u8,
    pub(crate) program: String,
    #[serde(rename = "formerClass")]
    pub(crate) former_class: bool,
    #[serde(rename = "regularCourse")]
    pub(crate) regular_course: bool,
    pub(crate) note: String,
}

impl Class {
    pub const fn new() -> Self {
        Self {
            date: String::new(),
            periods: Vec::new(),
            name: String::new(),
            teacher: String::new(),
            note: String::new(),
        }
    }
    pub fn parse(mut entry: &str) -> Result<Self, ()> {
        // DO NOT CHANGE THE PARSE ORDER.
        entry = entry.trim();
        let mut class: Class = Class::new();
        // note
        let note_regex = Regex::new(r"(【.+】)+$").unwrap();
        if let Some(note_index) = note_regex.find(entry) {
            let (other, note) = entry.split_at(note_index.start());
            entry = other.trim_end();
            let trimer: &[_] = &['【', '】'];
            // <note>, <note>
            class.note = note.trim_matches(trimer).replace("】【", ",");
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
            class.date = format!("{}-{}", month, day);
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
            // period_range: 要素は1つか2つ。2個の場合は
            let mut period_range: Vec<u8> = Vec::new();
            for c in periods.chars() {
                // n限目のnが2桁になることは想定していない
                if let Ok(number) = c.to_string().parse::<u8>() {
                    period_range.push(number);
                }
            }
            if period_range.len() == 1 {
                // 要素が1つの場合はそのまま追加
                class.periods.push(period_range[0]);
            } else if period_range.len() == 2 {
                // 要素が2つの場合は1つ目から2つ目までの自然数を列挙
                for i in period_range[0]..=period_range[1] {
                    class.periods.push(i);
                }
            } else {
                panic!("Period: {:?} is invalided value.", period_range);
            }
        };
        // teacher
        let teacher_regex = Regex::new(r"（.+）$").unwrap();
        if let Some(teacher_index) = teacher_regex.find(entry) {
            let (other, teacher) = entry.split_at(teacher_index.start());
            entry = other.trim_end();
            // Why not "trim_matches()"?
            // Cause:
            // > let trim_matches: &[_] = &['（', '）'];
            // > teacher.trim_matches(&trim_matches);
            // 上のコードでは以下の例のようなことが起こるため
            // 例: "（田中（太郎））" => "田中（太郎"
            // "（", "）": どちらも1byte文字ではないのでバイト列にして区切る地点を決めている
            let (_, teacher) = teacher.split_at("（".as_bytes().len());
            let (teacher, _) = teacher.split_at(teacher.as_bytes().len() - "）".as_bytes().len());
            class.teacher = teacher.to_string();
        }
        // name
        if !entry.is_empty() {
            class.name = entry.to_string();
        }
        // return Err(entry.to_string());
        Ok(class)
    }
}

impl ClassNumber {
    pub const fn new() -> Self {
        Self {
            grade: 1,
            program: String::new(),
            former_class: true,
            regular_course: true,
            note: String::new(),
        }
    }
    pub fn parse(mut entry: &str) -> Result<Self, ()> {
        let mut class_number = Self::new();
        // e.g. 4-S
        let entry_regex = Regex::new(r"(?P<class_number>((\w-\w)|(専.+))\S*)").unwrap();
        if let Some(c) = entry_regex.captures(entry) {
            entry = c.name("class_number").unwrap().as_str();
        }
        // e.g. （数学・物理科学プログラム）
        let program_regex = Regex::new(r"（(?P<program>.+プログラム)）").unwrap();
        if let Some(c) = program_regex.captures(entry) {
            class_number.note = c.name("program").unwrap().as_str().to_string();
        }
        // 新カリ
        if entry.starts_with(char::is_numeric) {
            // 学年
            match entry.chars().next().unwrap().to_digit(10) {
                Some(digit) => class_number.grade = digit as u8,
                None => return Err(()),
            }
            // クラス(本科1年生)
            if let Some(n) = entry.chars().last().unwrap().to_digit(10) {
                class_number.program = n.to_string();
            }
            // 系
            for c in entry.chars() {
                match c.to_string().as_str() {
                    "S" => class_number.program = "S".to_string(),
                    "M" => class_number.program = "M".to_string(),
                    "E" => class_number.program = "E".to_string(),
                    "C" => class_number.program = "C".to_string(),
                    _ => continue,
                }
            }
            class_number.former_class = false;
        // 旧カリ
        } else {
            if entry.contains('専') {
                // 専攻科
                class_number.program = "専".to_string();
                class_number.regular_course = false;
            } else {
                // 本科
                for c in entry.chars() {
                    match c.to_string().as_str() {
                        "S" => class_number.program = "S".to_string(),
                        "M" => class_number.program = "M".to_string(),
                        "E" => class_number.program = "E".to_string(),
                        "C" => class_number.program = "C".to_string(),
                        _ => continue,
                    }
                }
            }
            for c in entry.chars() {
                if c.is_numeric() {
                    class_number.grade = c.to_digit(10).unwrap() as u8;
                }
            }
            class_number.former_class = true;
        }
        Ok(class_number)
    }
}

#[cfg(test)]
mod test {
    use crate::class::{Class, ClassNumber};
    #[test]
    fn parse_date() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let result = Class::parse(sample).unwrap().date;
        assert_eq!(result, "12-05".to_string());
    }
    #[test]
    fn parse_period() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let result = Class::parse(sample).unwrap().periods;
        assert_eq!(result, [3, 4]);
    }
    #[test]
    fn parse_note() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let result = Class::parse(sample).unwrap().note;
        assert_eq!(result, "補講実施予定".to_string());
    }
    #[test]
    fn parse_teacher() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let result = Class::parse(sample).unwrap().teacher;
        assert_eq!(result, "吉田".to_string());
    }
    #[test]
    fn parse_name() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let result = Class::parse(sample).unwrap().name;
        assert_eq!(result, "集合と位相".to_string());
    }
    #[test]
    fn parse_class_name() {
        let sample = "12月5日(木) 4-S（数学・物理科学プログラム） [3・4限] 集合と位相（吉田）【補講実施予定】";
        let expected = ClassNumber {
            grade: 4,
            program: "S".to_string(),
            former_class: false,
            regular_course: true,
            note: "数学・物理科学プログラム".to_string(),
        };
        let result = ClassNumber::parse(sample).unwrap();
        assert_eq!(result, expected);
    }
}
