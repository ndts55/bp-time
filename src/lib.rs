use regex::Regex;
use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::fmt::{self, Display};

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

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Category::*;
        write!(
            f,
            "{}",
            match self {
                C => "Code",
                B => "BP",
                M => "Meet",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
struct Entry {
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

impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{}:\t{}h {}m", self.category, hours, minutes)
    }
}

pub struct Summary {
    entries: Vec<Entry>,
}

impl Summary {
    pub fn new(content: String) -> Self {
        let entries = summarize(content);
        Summary { entries }
    }

    fn total(&self) -> i32 {
        self.entries.iter().map(|entry| entry.minutes).sum()
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total = self.total();
        let total_hours = total / 60;
        let total_minutes = total % 60;
        write!(
            f,
            "{}",
            format!(
                "{}\nTotal:\t{}h {}m",
                self.entries
                    .iter()
                    .fold(String::new(), |acc, entry| format!("{}\n{}", acc, entry)),
                total_hours,
                total_minutes
            )
        )
    }
}

fn summarize(content: String) -> Vec<Entry> {
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
