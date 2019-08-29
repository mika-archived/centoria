// pub fn left_pad(string: &str, pad: usize) -> String {
//     let spaces = pad - string.len();
//     return format!("{}{}", " ".repeat(spaces), string);
// }

pub fn right_pad(string: &str, pad: usize) -> String {
    let spaces = pad - string.len();
    return format!("{}{}", string, " ".repeat(spaces));
}

#[test]
fn test_format() {
    // assert_eq!(left_pad("Hello", 5), "Hello");
    // assert_eq!(left_pad("Hello", 10), "     Hello");
    assert_eq!(right_pad("Hello", 5), "Hello");
    assert_eq!(right_pad("Hello", 10), "Hello     ");
}
