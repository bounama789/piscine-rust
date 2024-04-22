pub fn fibonacci(n: u32) -> u32 {
    match n {
       0 => 0,
       1 => 1,
       _ => fibonacci(n - 1) + fibonacci(n -2) 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let result = fibonacci(4);
        assert_eq!(result, 3);
    }
}
