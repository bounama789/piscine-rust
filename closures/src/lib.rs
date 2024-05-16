pub fn first_fifty_even_square() -> Vec<i32> {
    (1..).into_iter().filter(|n| n%2==0).map(|c| c*c).take(50).collect()
}
