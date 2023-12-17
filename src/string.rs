pub fn align_string(left: &str, right: String) -> String {
    let width = 32;
    let space_count = width - left.len();

    format!("{}{}{}", left, " ".repeat(space_count), right)
}
