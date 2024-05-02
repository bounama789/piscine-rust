pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        "Just say something!"
    } else if text == text.trim().to_uppercase() {
        if text.trim().ends_with('?') {
            if text.contains(char::is_numeric)  {
                "Sure."
            } else {
                "Quiet, I am thinking!"
            }
        } else {
            "There is no need to yell, calm down!"
        }
    } else if text.trim().ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }
}
