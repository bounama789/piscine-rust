pub fn first_subword(mut s: String) -> String {
    match s
        .char_indices()
        .find(|(idx, c)| *idx > 0 && (!c.is_alphabetic() || c.is_uppercase()))
    {
        Some((idx,_)) => s =s[..idx].to_string(),
        None => s = s,
    };
    return s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_subword() {
        let s1 = String::from("helloWorld");
        let s2 = String::from("snake_case");
        let s3 = String::from("CamelCase");
        let s4 = String::from("just");

        let result = first_subword(first_subword(s1));
        assert_eq!(result, "hello");
        let result = first_subword(first_subword(s2));
        assert_eq!(result, "snake");
        let result = first_subword(first_subword(s3));
        assert_eq!(result, "Camel");
        let result = first_subword(first_subword(s4));
        assert_eq!(result, "just");
    }
}
