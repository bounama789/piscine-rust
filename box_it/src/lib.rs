pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let tokens: Vec<u32> = s
        .split_ascii_whitespace()
        .into_iter()
        .map(|t| {
            let mut t = t.to_string();
            if t.chars().last().unwrap() == 'k' {
                t.pop();
                let int: f64 = t.parse().expect("error while parsing");
                (int*1000.0) as u32
            } else {
                let int: u32 = t.parse().expect("error while parsing");
                int
            }
        })
        .collect();

    Box::new(tokens)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_and_save_on_heap() {
        let new_str = String::from("5.5k 8.9k 32");

        // creating a variable and we save it in the Heap
        let a_h = transform_and_save_on_heap(new_str);
        assert_eq!(a_h,Box::new(vec![5500, 8900, 32]));
        assert_eq!(std::mem::size_of_val(&a_h),8)
    }

    #[test]
    fn test_take_value_ownership() {
        let new_str = String::from("5.5k 8.9k 32");

        // creating a variable and we save it in the Heap
        let a_h = transform_and_save_on_heap(new_str);
        let res = take_value_ownership(a_h);
        assert_eq!(res,vec![5500, 8900, 32]);
        assert_eq!(std::mem::size_of_val(&res),24)
    }
}
