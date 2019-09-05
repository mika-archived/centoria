#[allow(dead_code)]
pub fn left_pad(string: &str, pad: usize) -> String {
    let spaces = checked_calculate(pad, string.len());
    return format!("{}{}", " ".repeat(spaces), string);
}

pub fn left_pad_without_1st(string: &str, pad: usize) -> String {
    // oh...
    return string
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
        .map(|(i, s)| format!("{}{}", " ".repeat(if i == 0 { 0 } else { pad }), s))
        .collect::<Vec<String>>()
        .join("\n")
        .trim()
        .to_owned();
}

pub fn right_pad(string: &str, pad: usize) -> String {
    let spaces = checked_calculate(pad, string.len());
    return format!("{}{}", string, " ".repeat(spaces));
}

fn checked_calculate(s1: usize, s2: usize) -> usize {
    return match s1.checked_sub(s2) {
        Some(value) => value,
        None => 0,
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn left_pad() {
        assert_eq!(super::left_pad("Hello", 5), "Hello");
        assert_eq!(super::left_pad("Hello", 10), "     Hello");
        assert_eq!(super::left_pad("Hello", 2), "Hello"); // not effect
    }

    #[test]
    fn left_pad_without_1st() {
        assert_eq!(
            super::left_pad_without_1st(
                "\
     Hello
Hello",
                5
            ),
            "\
     Hello
     Hello"
        );
    }

    #[test]
    fn right_pad() {
        assert_eq!(super::right_pad("Hello", 5), "Hello");
        assert_eq!(super::right_pad("Hello", 10), "Hello     ");
        assert_eq!(super::right_pad("Hello", 2), "Hello"); // not effect
    }
}
