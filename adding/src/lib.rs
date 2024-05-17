pub fn add_curry(a: i32) -> impl Fn(i32) -> i32 {
    return move |b| a + b;
}

mod tests {
    use super::*;
    #[test]
    fn test_add_curry() {
        let add10 = add_curry(-10);
        let add20 = add_curry(2066);
        let add30 = add_curry(300000);



        assert_eq!(add10(5), -5);
        assert_eq!(add20(195), 2261);
        assert_eq!(add30(5696), 305696);
    }
}
