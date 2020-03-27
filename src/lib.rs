use regex::Regex;
use std::cmp::PartialEq;

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

struct Entry {
    category: Category,
    minutes: i32,
}

fn summarize(content: String) -> Vec<Entry> {
    content
        .lines()
        .filter_map(parse_line)
        .fold(Vec::new(), sum_by_category)
}

fn parse_line(line: &str) -> Option<Entry> {
    None
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
