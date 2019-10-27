pub fn to_single_line(string: &str) -> String {
    string.replace("\n", " ").trim().to_owned()
}
