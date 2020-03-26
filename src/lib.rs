use regex::Regex;

static ENTRY_PATTERN: &str =
    r"((?P<hours>\d+)h\s*((?P<minutes>\d+)m)?|(?P<minutes_only>\d+)m)\s*\|\s*(?P<category>(m|b|c))";

lazy_static::lazy_static! {
    static ref ENTRY_REGEX: Regex = Regex::new(ENTRY_PATTERN).unwrap();
}

