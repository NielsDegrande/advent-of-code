fn reverse_string(string: &str) -> String {
    string.chars().rev().collect()
}

fn remove_whitespace(string: &str) -> String {
    string.chars().filter(|c| !c.is_whitespace()).collect()
}
