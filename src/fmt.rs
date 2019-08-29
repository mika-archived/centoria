pub fn to_single_line(string: &str) -> String {
    return string.replace("\n", " ");
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
