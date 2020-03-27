use regex::Regex;
use std::cmp::PartialEq;
use std::convert::TryFrom;

static ENTRY_PATTERN: &str =
    r"((?P<hours>\d+)h\s*((?P<minutes>\d+)m)?|(?P<minutes_only>\d+)m)\s*\|\s*(?P<category>(m|b|c))";

lazy_static::lazy_static! {
    static ref ENTRY_REGEX: Regex = Regex::new(ENTRY_PATTERN).unwrap();
}

#[derive(PartialEq)]
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

fn summarize(content: String) -> Vec<Entry> {
    content
        .lines()
        .filter_map(parse_line)
        .fold(Vec::new(), sum_by_category)
}

fn parse_line(line: &str) -> Option<Entry> {
    let get_num = |caps: &regex::Captures, name: &str| {
        caps.name(name)
            .and_then(|ma| ma.as_str().parse::<i32>().ok().or(Some(0)))
    };
    ENTRY_REGEX
        .captures(line)
        .and_then(|caps| {
            caps.name("category")
                .and_then(|ma| Category::try_from(ma.as_str()).ok())
                .map(|category| (caps, category))
        })
        .and_then(|(caps, category)| {
            get_num(&caps, "hours").map(|h| (caps, Entry::new(category, h * 60)))
        })
        .and_then(|(caps, entry)| {
            get_num(&caps, "minutes").map(|minutes| (caps, entry.add(minutes)))
        })
        .and_then(|(caps, entry)| {
            get_num(&caps, "minutes_only").map(|minutes| entry.add(minutes))
        })
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
