pub fn str_len(s: &str) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_len() {
        let s = "hello";
	let s1 = "camelCase".to_string();

        let result = str_len(s);
        assert_eq!(result, 5);
        let result = str_len(&s1);
        assert_eq!(result, 9);
    }
}
