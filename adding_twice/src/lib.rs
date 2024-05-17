use adding::add_curry;

pub fn twice<T>(f: T) -> impl Fn(i32) -> i32
where
    T: Fn(i32) -> i32,
{
    let f = move |b| {
        let a = f(b);
        return add_curry((a - b) * 2)(b);
    };
    f
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_twice() {
        let add10 = add_curry(10);
        let value = twice(add10);
        assert_eq!(value(7), 27);

        let add20 = add_curry(20);
        let value = twice(add20);
        assert_eq!(value(7), 47);

        let add30 = add_curry(30);
        let value = twice(add30);
        assert_eq!(value(7), 67);

        let neg = add_curry(-32);
        let value = twice(neg);
        assert_eq!(value(7), -57);
    }
}
