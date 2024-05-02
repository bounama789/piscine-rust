pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        "Just say something!"
    } else if text == text.to_uppercase() {
        if text.ends_with('?') {
            "Quiet, I am thinking!"
        } else {
            "There is no need to yell, calm down!"
        }
    } else if text.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }
}
