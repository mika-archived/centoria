use std::fmt::Display;

use regex::{Captures, Regex};

pub fn format_array<T>(
    string: &str,
    variable: &str,
    array: &Vec<T>,
) -> Result<String, failure::Error>
where
    T: Display,
{
    // variable{1}, variable{1?}, variable{1..}, variable{1..2}, variable{1..?}
    let variable = format!(
        r"{}\{{((?P<index>\d+)(?P<optional_idx>\?)?|(?P<start>\d+)\.\.((?P<end>(\d+|\?)))?)\}}",
        variable
    );
    let variable = Regex::new(&variable).unwrap();

    // if does not exists variable in string
    if !variable.is_match(string) {
        return Ok(string.to_owned());
    }

    let mut replaced = string.to_owned();
    for capture in variable.captures_iter(&string) {
        if let Some(_) = capture.name("index") {
            // single index
            replaced = format_single_index(replaced, &array, capture)?;
        } else if let Some(_) = capture.name("start") {
            // range index
            replaced = format_range_index(replaced, &array, capture)?;
        } else {
            return Err(failure::err_msg("not implemented yet (unknown pattern)"));
        }
    }

    return Ok(replaced.to_owned());
}

fn format_single_index<T>(
    string: String,
    array: &Vec<T>,
    captures: Captures,
) -> Result<String, failure::Error>
where
    T: Display,
{
    let index: usize = captures.name("index").unwrap().as_str().parse().unwrap();
    let optional: bool = match captures.name("optional_idx") {
        Some(value) => value.as_str() == "?", // always true?
        None => false,
    };

    let from = captures.get(0).unwrap().as_str();
    let to = array
        .get(index)
        .map_or_else(|| optional_param(optional), |w| Ok(w.to_string()));
    match to {
        Ok(value) => return Ok(string.replace(from, value.to_owned().as_str())),
        Err(e) => return Err(failure::err_msg(e)),
    };
}

fn format_range_index<T>(
    string: String,
    array: &Vec<T>,
    captures: Captures,
) -> Result<String, failure::Error>
where
    T: Display,
{
    let start: usize = captures.name("start").unwrap().as_str().parse().unwrap();
    let end = captures
        .name("end")
        .map_or_else(|| array.len().to_string(), |v| v.as_str().to_owned());
    let from = captures.get(0).unwrap().as_str();

    if let Ok(index) = end.parse::<usize>() {
        let to = match array.get(start..index) {
            Some(values) => values
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            None => return Err(failure::err_msg("index out of bounds or invalid access")),
        };

        return Ok(string.replace(from, to.join(" ").as_str()));
    } else if end == "?" {
        let to = match array.get(start..array.len()) {
            Some(values) => values
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            None => vec![],
        };

        return Ok(string.replace(from, to.join(" ").as_str()));
    } else {
        return Err(failure::err_msg(format!("invalid accessor: {}", end)));
    }
}

fn optional_param(has_optional: bool) -> Result<String, failure::Error> {
    if has_optional {
        return Ok("".to_owned());
    } else {
        return Err(failure::err_msg("index out of bounds"));
    }
}

#[test]
fn test_for_array_indexes() {
    let array = vec!["World", "John", "Alice"];

    assert_eq!(
        format_array("Hello, args{0}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, World!"
    );

    assert_eq!(
        format_array("Hello, args{1}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, John!"
    );

    assert_eq!(
        format_array("Hello, args{2}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, Alice!"
    );

    assert_eq!(
        format_array("Hello, args{1}, args{2}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, John, Alice!"
    );

    assert_eq!(
        format_array("Hello, args{3}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "index out of bounds"
    );

    assert_eq!(
        format_array("Hello, args{a}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, args{a}!" // not replacement
    );
}

#[test]
fn test_for_array_ranges() {
    let array = vec!["John", "Smith"];

    assert_eq!(
        format_array("Hello, args{0..1}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, John!"
    );

    assert_eq!(
        format_array("Hello, args{0..2}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, John Smith!"
    );

    assert_eq!(
        format_array("Hello, args{0..}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, John Smith!"
    );

    assert_eq!(
        format_array("Hello, args{1..}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, Smith!"
    );

    assert_eq!(
        format_array("Hello, args{1..0}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "index out of bounds or invalid access"
    );

    assert_eq!(
        format_array("Hello, args{0..3}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "index out of bounds or invalid access"
    );

    assert_eq!(
        format_array("Hello, args{0..a}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, args{0..a}!" // not replacement
    );
}

#[test]
fn test_for_optional() {
    let array = vec!["John", "Smith"];

    assert_eq!(
        format_array("Hello, args{1?}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, Smith!"
    );

    assert_eq!(
        format_array("Hello, args{2?}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, !"
    );

    assert_eq!(
        format_array("Hello, args{0..?}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, John Smith!"
    );

    assert_eq!(
        format_array("Hello, args{2..?}!", "args", &array).unwrap_or_else(|e| e.to_string()),
        "Hello, !"
    );
}
