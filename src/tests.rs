#[allow(dead_code)]
#[allow(unused_imports)]
mod sum_by_category_tests {
    use super::super::*;

    #[test]
    fn new_category() {
        let v = vec![Entry::new(Category::B, 30), Entry::new(Category::M, 10)];
        let entry = Entry::new(Category::C, 90);
        let expected = vec![
            Entry::new(Category::B, 30),
            Entry::new(Category::M, 10),
            Entry::new(Category::C, 90),
        ];

        assert_array_eq(expected, sum_by_category(v, entry));
    }

    #[test]
    fn existing_category() {
        let v = vec![
            Entry::new(Category::B, 30),
            Entry::new(Category::M, 10),
            Entry::new(Category::C, 90),
        ];
        let entry = Entry::new(Category::C, 10);
        let expected = vec![
            Entry::new(Category::B, 30),
            Entry::new(Category::M, 10),
            Entry::new(Category::C, 100),
        ];

        assert_array_eq(expected, sum_by_category(v, entry));
    }

    #[test]
    fn as_fold_function() {
        let v = vec![
            Entry::new(Category::C, 10),
            Entry::new(Category::C, 10),
            Entry::new(Category::C, 10),
            Entry::new(Category::C, 10),
        ];
        let expected = vec![Entry::new(Category::C, 40)];

        let actual = v.into_iter().fold(Vec::new(), sum_by_category);

        assert_array_eq(expected, actual);
    }

    fn assert_array_eq(expected: Vec<Entry>, actual: Vec<Entry>) {
        for (e, a) in expected.iter().zip(actual.iter()) {
            assert_eq!(*e, *a);
        }
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
mod parse_line_tests {
    use super::super::*;

    #[test]
    fn no_match() {
        let line = "this line is not a match";
        let expected = None;
        let actual = parse_line(line);

        assert_eq!(expected, actual);
    }

    #[test]
    fn match_hour_and_minute() {
        let line = "| 2020-01-02 | 14.00 | 16.45 | 2h 45m | c BP-457 |";
        let expected = Some(Entry::new(Category::C, 165));
        let actual = parse_line(line);

        assert_eq!(expected, actual);
    }

    #[test]
    fn match_hours() {
        let line = "| 2020-01-05 | 15.00 | 19.00 | 4h | c BP-457 |";
        let expected = Some(Entry::new(Category::C, 240));
        let actual = parse_line(line);

        assert_eq!(expected, actual);
    }

    #[test]
    fn match_minutes() {
        let line = "| 2020-01-15 | 12.45 | 13.10 | 25m | c BP-505 |";
        let expected = Some(Entry::new(Category::C, 25));
        let actual = parse_line(line);

        assert_eq!(expected, actual);
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
mod entry_regex_tests {
    use super::super::*;

    #[test]
    fn no_match() {
        let line = "this line is not a match";
        let actual = ENTRY_REGEX.captures(line);

        assert!(actual.is_none());
    }

    #[test]
    fn match_hours_and_minutes() {
        let line = "| 2020-01-02 | 14.00 | 16.45 | 2h 45m | c BP-457 |";
        let actual = ENTRY_REGEX.captures(line);

        assert!(actual.is_some());

        let actual = actual.unwrap();
        let category = actual.name("category");
        let hours = actual.name("hours");
        let minutes = actual.name("minutes");

        assert!(category.is_some());
        assert!(hours.is_some());
        assert!(minutes.is_some());

        let category = category.unwrap().as_str();
        let hours = hours.unwrap().as_str();
        let minutes = minutes.unwrap().as_str();

        assert_eq!("c", category);
        assert_eq!("2", hours);
        assert_eq!("45", minutes);
    }

    #[test]
    fn match_hours() {
        let line = "| 2020-01-05 | 15.00 | 19.00 | 4h | c BP-457 |";
        let actual = ENTRY_REGEX.captures(line);

        assert!(actual.is_some());

        let actual = actual.unwrap();
        let category = actual.name("category");
        let hours = actual.name("hours");

        assert!(category.is_some());
        assert!(hours.is_some());

        let category = category.unwrap().as_str();
        let hours = hours.unwrap().as_str();

        assert_eq!("c", category);
        assert_eq!("4", hours);
    }

    #[test]
    fn match_minutes() {
        let line = "| 2020-01-15 | 12.45 | 13.10 | 25m | c BP-505 |";
        let actual = ENTRY_REGEX.captures(line);

        assert!(actual.is_some());

        let actual = actual.unwrap();
        let category = actual.name("category");
        let minutes = actual.name("minutes_only");

        assert!(category.is_some());
        assert!(minutes.is_some());

        let category = category.unwrap().as_str();
        let minutes = minutes.unwrap().as_str();

        assert_eq!("c", category);
        assert_eq!("25", minutes);
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
mod category_try_from_tests {
    use super::super::*;

    #[test]
    fn from_c() {
        let expected = Ok(Category::C);
        let actual = Category::try_from("c");

        assert_eq!(expected, actual);
    }

    #[test]
    fn from_b() {
        let expected = Ok(Category::B);
        let actual = Category::try_from("b");

        assert_eq!(expected, actual);
    }

    #[test]
    fn from_m() {
        let expected = Ok(Category::M);
        let actual = Category::try_from("m");

        assert_eq!(expected, actual);
    }

    #[test]
    fn from_invalid() {
        let expected = Err("invalid category");
        let actual = Category::try_from("z");

        assert_eq!(expected, actual);
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
mod parse_number_tests {
    use super::super::*;

    #[test]
    fn parse_hours_and_minutes() {
        let line = "| 2020-01-02 | 14.00 | 16.45 | 2h 45m | c BP-457 |";
        let caps = ENTRY_REGEX.captures(line).unwrap();

        let hours = parse_number(caps.name("hours").unwrap());
        let minutes = parse_number(caps.name("minutes").unwrap());

        assert_eq!(Some(2), hours);
        assert_eq!(Some(45), minutes);
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
mod get_num_tests {
    use super::super::*;

    #[test]
    fn get_hours_and_minutes() {
        let line = "| 2020-01-02 | 14.00 | 16.45 | 2h 45m | c BP-457 |";
        let caps = ENTRY_REGEX.captures(line).unwrap();

        let hours = get_num_or_default(&caps, "hours");
        let minutes = get_num_or_default(&caps, "minutes");

        assert_eq!(2, hours);
        assert_eq!(45, minutes);
    }

    #[test]
    fn get_no_match_for_minutes_only() {
        let line = "| 2020-01-02 | 14.00 | 16.45 | 2h 45m | c BP-457 |";
        let caps = ENTRY_REGEX.captures(line).unwrap();

        let minutes = get_num_or_default(&caps, "minutes_only");

        assert_eq!(0, minutes);
    }
}
