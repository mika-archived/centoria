use std::fmt::Display;

use regex::Regex;

pub fn format_array<T>(
    string: &str,
    variable: &str,
    array: &Vec<T>,
) -> Result<String, failure::Error>
where
    T: Display,
{
    // variable[1], variable[1..2], variable[1..]
    let variable = format!(
        r"{}\{{(?P<index>\d+)(?P<range>\.\.(?P<end>\d+)?)?\}}",
        variable
    );
    let variable = Regex::new(&variable).unwrap();

    // if does not exists variable in string
    if !variable.is_match(string) {
        return Ok(string.to_owned());
    }

    let mut replaced = string.to_owned();
    for capture in variable.captures_iter(&string) {
        let index: usize = capture.name("index").unwrap().as_str().parse().unwrap();
        if index >= array.len() {
            return Err(failure::err_msg("index out of bounds"));
        }

        // ranges
        if let Some(_) = capture.name("range") {
            let range: usize = match capture.name("end") {
                Some(value) => value.as_str().parse().unwrap(),
                None => array.len(),
            };

            if range <= index || range >= array.len() + 1 {
                return Err(failure::err_msg("index out of bounds"));
            }

            let from = capture.get(0).unwrap().as_str();
            let to = array[index..range]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            replaced = replaced.replace(from, to.join(" ").as_str());
        } else {
            let from = capture.get(0).unwrap().as_str();
            let to = array.get(index).unwrap();
            replaced = replaced.replace(from, to.to_string().as_str());
        }
    }

    return Ok(replaced.to_owned());
}

#[test]
fn test_for_array_indexes() {
    let array = vec!["World", "John", "Alice"];

    assert_eq!(
        format_array("Hello, args{0}!", "args", &array).unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        format_array("Hello, args{1}!", "args", &array).unwrap(),
        "Hello, John!"
    );

    assert_eq!(
        format_array("Hello, args{2}!", "args", &array).unwrap(),
        "Hello, Alice!"
    );

    assert_eq!(
        format_array("Hello, args{3}!", "args", &array).unwrap_or("Error".to_owned()),
        "Error"
    );
}

#[test]
fn test_for_array_ranges() {
    let array = vec!["John", "Smith"];

    assert_eq!(
        format_array("Hello, args{0..1}!", "args", &array).unwrap(),
        "Hello, John!"
    );

    assert_eq!(
        format_array("Hello, args{0..2}!", "args", &array).unwrap(),
        "Hello, John Smith!"
    );

    assert_eq!(
        format_array("Hello, args{0..}!", "args", &array).unwrap(),
        "Hello, John Smith!"
    );

    assert_eq!(
        format_array("Hello, args{1..}!", "args", &array).unwrap(),
        "Hello, Smith!"
    );

    assert_eq!(
        format_array("Hello, args{1..0}!", "args", &array).unwrap_or("Error".to_owned()),
        "Error"
    );

    assert_eq!(
        format_array("Hello, args{0..3}!", "args", &array).unwrap_or("Error".to_owned()),
        "Error"
    );
}
