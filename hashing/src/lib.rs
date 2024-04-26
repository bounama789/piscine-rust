use std::collections::HashMap;

use arrays::sum;

pub fn mean(list: &Vec<i32>) -> f64 {
    sum(&list) as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut l = list.clone();
    l.sort();
    println!("{:?}",l);
    match l.len() % 2 {
        0 => (l[l.len() / 2] + l[l.len() / 2 + 1]) / 2,
        _ => l[l.len() / 2 ],
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut num_frequency = HashMap::new();
    list.iter().for_each(|&num| {
        *num_frequency.entry(num).or_insert(0) += 1;
    });

    num_frequency
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
        .expect("list is empty")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let v = vec![4, 7, 5, 2, 5, 1, 3];
        let result = mean(&v);
        assert_eq!(result, 3.857142857142857);
    }

    #[test]
    fn test_median() {
        let v = vec![4, 7, 5, 2, 5, 1, 3];

        let result = median(&v);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_mode() {
        let v = vec![4, 7, 5, 2, 5, 1, 3];

        let result = mode(&v);
        assert_eq!(result, 5);
    }
}
