
pub fn sum(a: &[i32]) -> i32 {
    let sum =a.iter().reduce(|acc, e| &(acc +e)).unwrap();
    return sum;
}


pub fn thirtytwo_tens() -> [i32; 32] {
    [10 as i32; 32]
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
