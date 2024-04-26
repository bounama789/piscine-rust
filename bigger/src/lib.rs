use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
   h.into_values().max().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigger() {
        let mut hash = HashMap::new();
        hash.insert("Daniel", 122);
        hash.insert("Ashley", 333);
        hash.insert("Katie", 334);
        hash.insert("Robert", 14);

        let result = bigger(hash);

        assert_eq!(result, 334);
    }
}
