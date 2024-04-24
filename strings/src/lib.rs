pub fn char_length(s: &str) -> usize {
    s.chars().count()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_length() {
        let result = char_length("❤");
        assert_eq!(result, 1);

        let result = char_length("形聲字");
        assert_eq!(result, 3);

        let result = char_length("change");
        assert_eq!(result, 6);

        let result = char_length("😍");
        assert_eq!(result, 1);
    }
}
