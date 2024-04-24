pub fn capitalize_first(input: &str) -> String {
    input
        .char_indices()
        .map(|(idx, c)| {
            if idx == 0 {
                return c.to_uppercase().to_string();
            }
            return c.to_string();
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                return c.to_uppercase().to_string();
            } else if c.is_uppercase() {
                return c.to_lowercase().to_string();
            }
            return c.to_string();
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        let result = capitalize_first("joe is missing");
        assert_eq!(result, "Joe is missing");
    }

    #[test]
    fn test_title_case() {
        let result = title_case("joe is missing");
        assert_eq!(result, "Joe Is Missing");
    }

    #[test]
    fn test_change_case() {
        let result = change_case("heLLo THere");
        assert_eq!(result, "HEllO thERE");
    }

    
}
