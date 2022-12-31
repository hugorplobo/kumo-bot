pub fn escape_filename(name: &str) -> String {
    let res = String::from(name);
    res.replace(".", "\\.")
}