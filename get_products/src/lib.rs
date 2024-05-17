pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    if n == 0 {
        return vec![];
    }

    let mut result = vec![1; n];

    let mut prefix = 1;
    for i in 0..n {
        result[i] = prefix;
        prefix *= arr[i];
        println!("{:#?}",result);
    }
    //[1,1,2]

    let mut suffix = 1;
    for i in (0..n).rev() {
        result[i] *= suffix;
        suffix *= arr[i];
    }
    //[3,3,2]

    result
}
