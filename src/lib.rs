use regex::Regex;
use std::cmp::PartialEq;
use std::convert::TryFrom;

mod tests;

static ENTRY_PATTERN: &str =
    r"((?P<hours>\d+)h\s*((?P<minutes>\d+)m)?|(?P<minutes_only>\d+)m)\s*\|\s*(?P<category>(m|b|c))";

lazy_static::lazy_static! {
    static ref ENTRY_REGEX: Regex = Regex::new(ENTRY_PATTERN).unwrap();
}

#[derive(Debug, PartialEq)]
enum Category {
    C,
    B,
    M,
}

impl TryFrom<&str> for Category {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Category::*;
        match value {
            "c" => Ok(C),
            "b" => Ok(B),
            "m" => Ok(M),
            _ => Err("invalid category"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Entry {
    category: Category,
    minutes: i32,
}

impl Entry {
    fn new(category: Category, minutes: i32) -> Self {
        Entry { category, minutes }
    }

    fn add(self, additional_minutes: i32) -> Self {
        Entry {
            minutes: additional_minutes + self.minutes,
            ..self
        }
    }
}

pub fn summarize(content: String) -> Vec<Entry> {
    content
        .lines()
        .filter_map(parse_line)
        .fold(Vec::new(), sum_by_category)
}

fn parse_line(line: &str) -> Option<Entry> {
    ENTRY_REGEX
        .captures(line)
        .and_then(|caps| {
            caps.name("category")
                .and_then(|ma| Category::try_from(ma.as_str()).ok())
                .map(|category| (caps, category))
        })
        .map(|(caps, category)| {
            let h = get_num_or_default(&caps, "hours");
            (caps, Entry::new(category, h * 60))
        })
        .map(|(caps, entry)| {
            let minutes = get_num_or_default(&caps, "minutes");
            (caps, entry.add(minutes))
        })
        .map(|(caps, entry)| {
            let minutes = get_num_or_default(&caps, "minutes_only");
            entry.add(minutes)
        })
}

fn get_num_or_default(caps: &regex::Captures, name: &str) -> i32 {
    caps.name(name).and_then(parse_number).unwrap_or_default()
}

fn parse_number(ma: regex::Match) -> Option<i32> {
    ma.as_str().parse::<i32>().ok()
}

fn sum_by_category(acc: Vec<Entry>, entry: Entry) -> Vec<Entry> {
    let mut acc = acc;
    if let Some(index) = acc
        .iter()
        .position(|element| element.category == entry.category)
    {
        acc[index].minutes += entry.minutes;
    } else {
        acc.push(entry);
    }

    acc
}
