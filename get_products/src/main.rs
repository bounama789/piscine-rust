use get_products::get_products;

fn main() {
    let arr: Vec<usize> = vec![4, 8, 2];
    let output = get_products(arr);
    println!("{:?}", output);
}