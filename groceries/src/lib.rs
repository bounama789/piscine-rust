pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    vec[index].to_string()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_insert() {
        let mut groceries = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
        ];
        let results = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
            "nuts".to_string(),
        ];
        insert(&mut groceries, String::from("nuts"));
        assert_eq!(groceries, results)
    }

    #[test]
    fn test_at_index() {
        let groceries = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
        ];
        let result = at_index(&groceries, 1);
        assert_eq!(result,"panettone")
    }
}
