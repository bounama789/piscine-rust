pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();

    for c in s.chars() {
        if c == '-' {
            result.pop();
        } else {
            result.push(c);
        }
    }

    let mut final_result = String::new();
    for c in result.chars().rev() {
        if c == '+' {
            final_result.pop();
        }else {
            final_result.push(c);
        }
    }

    *s = final_result.chars().rev().collect();
}

pub fn do_operations(v: &mut Vec<String>) {
    for i in 0..v.len() {
        let parts = v[i].split(|c: char| !c.is_numeric())
            .filter_map(|n| n.parse::<i32>().ok())
            .collect::<Vec<_>>();

        if v[i].contains('+') {
            v[i] = (parts[0] + parts[1]).to_string();
        } else if v[i].contains('-') {
            v[i] = (parts[0] - parts[1]).to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_and_backspace() {
        let mut a = String::from("bpp--o+er+++sskroi-++lcw");
        delete_and_backspace(&mut a);
        assert_eq!(a, "borrow");
    }

    #[test]
    fn test_do_operations() {
        let mut b: Vec<String> = vec![
            "2+2".to_string(),
            "3+2".to_string(),
            "10-3".to_string(),
            "5+5".to_string(),
        ];
        do_operations(&mut b);
        assert_eq!(b, ["4", "5", "7", "10"]);
    }
}
