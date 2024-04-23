pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
   match v.find(pat) {
       Some(index) => index,
       None => v.len()
   }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let result = is_empty("");
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_ascii() {
        let result = is_ascii("rust");
        assert_eq!(result, true);
    }

    #[test]
    fn test_contains() {
        let result = contains("rust","ru");
        assert_eq!(result, true);
    }

    #[test]
    fn test_split_at() {
        let result = split_at("rust",2);
        assert_eq!(result, ("ru", "st"));
    }

    #[test]
    fn test_find() {
        let result = find("rust",'u');
        assert_eq!(result, 1);
    }
    
}
