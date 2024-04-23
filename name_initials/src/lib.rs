pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_ascii_whitespace()
                .filter_map(|word| word.chars().next().map(|c| format!("{}.", c)))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initials() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        let result = initials(names);
        assert_eq!(result, ["H. P.", "S. E.", "J. L.", "B. O."]);
    }
}
