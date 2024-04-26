pub fn bubble_sort(vec: &mut Vec<i32>) {
    vec.sort()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let ref mut v = vec![3, 2, 4, 5, 1, 7];
        let mut b = v.clone();
        bubble_sort(v);
        b.sort();
        assert_eq!(*v, b);
    }
}
