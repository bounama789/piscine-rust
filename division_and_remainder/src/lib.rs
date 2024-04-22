pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let (quo,rem) = (x/y,x%y);
    (quo,rem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        let result = divide(9, 4);
    
        assert_eq!(result, (2,1));
    }
}
