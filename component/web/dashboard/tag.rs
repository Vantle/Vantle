pub const FUNCTIONAL: &str = "functional";
pub const PERFORMANCE: &str = "performance";
pub const FAILING: &str = "failing";
pub const PASS: &str = "pass";
pub const FAIL: &str = "fail";

#[must_use]
pub fn contains(tags: &[String], value: &str) -> bool {
    tags.iter().any(|t| t == value)
}

pub fn insert(tags: &mut Vec<String>, value: &str) {
    if !contains(tags, value) {
        tags.push(value.to_string());
    }
}

pub fn prepend(tags: &mut Vec<String>, value: &str) {
    if !contains(tags, value) {
        tags.insert(0, value.to_string());
    }
}
