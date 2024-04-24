pub fn sum(a: &[i32]) -> i32 {
    a.iter().fold(0,|acc, &e,| acc +e)
}


pub fn thirtytwo_tens() -> [i32; 32] {
    [10 as i32; 32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let result = sum(&a);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_thirtytwo_tens() {
        let a = [10;32];

        let result = thirtytwo_tens();
        assert_eq!(result, a);
        assert_eq!(result.len(),a.len())
    }
}
