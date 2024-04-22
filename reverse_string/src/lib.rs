pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rev_str() {
        let result = rev_str("Hello, world!");
        assert_eq!(result, "!dlrow ,olleH");

        let result = rev_str("Hello, my name is Roman");
        assert_eq!(result, "namoR si eman ym ,olleH");

        let result = rev_str("I have a nice car!");
        assert_eq!(result, "!rac ecin a evah I");

        let result = rev_str("How old are You");
        assert_eq!(result, "uoY era dlo woH");

        let result = rev_str("ex: this is an example água");
        assert_eq!(result, "augá elpmaxe na si siht :xe");
    }
}
