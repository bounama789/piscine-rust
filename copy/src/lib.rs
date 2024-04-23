pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let f = c as f64;
    (c, f.exp(), f.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let r = a
        .split_ascii_whitespace()
        .map(|x| {
            let f: f64 = x.parse().unwrap();
            return f.exp().to_string();
        })
        .collect::<Vec<_>>()
        .join(" ");

    (a, r)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let r = b.iter().map(|x| (*x as f64).abs().ln()).collect();
    (b, r)
}

#[cfg(test)]
mod tests {
    use std::f64::NEG_INFINITY;

    use super::*;

    #[test]
    fn test_nbr_function() {
        let a: i32 = 0;
        let result = nbr_function(a);
        assert_eq!(result, (0, 1.0, NEG_INFINITY));
    }

    #[test]
    fn test_str_function() {
        let b = String::from("1 2 4 5 6");
        let result = str_function(b);
        assert_eq!(result, ("1 2 4 5 6".to_string(), "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_string()));
    }

    #[test]
    fn test_vec_function() {
        let c = vec![1, 2, 4, 5];
        let result = vec_function(c);
        assert_eq!(
            result,
            (
                vec![1, 2, 4, 5],
                vec![
                    0.0,
                    0.6931471805599453,
                    1.3862943611198906,
                    1.6094379124341003
                ]
            )
        );
    }
}
