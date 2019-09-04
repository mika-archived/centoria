pub fn to_single_line(string: &str) -> String {
    return string.replace("\n", " ").trim().to_owned();
}
