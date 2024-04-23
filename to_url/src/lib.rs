pub fn to_url(s: &str) -> String {
    s.replace(' ', "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_url() {
        let s = "Hello, world!";
        let result = to_url(s);
        assert_eq!(result, "Hello,%20world!");
    }
}
