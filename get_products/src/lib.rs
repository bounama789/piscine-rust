pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    if n == 0 {
        return vec![];
    }

    let mut result = vec![1; n];

    //example: [4,8,2]

    let mut prefix = 1;
    for i in 0..n {
        result[i] = prefix;
        prefix *= arr[i];
    }
    //[1,4,32]

    let mut suffix = 1;
    for i in (0..n).rev() {
        result[i] *= suffix;
        suffix *= arr[i];
    }
    // final result: [16,8,32]

    result
}
