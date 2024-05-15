use matrix::*;

fn main() {
    let m: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
    println!("{:?}", m);
    println!("{:?}", Matrix::<i32>::identity(4));
    println!("{:?}", Matrix::<f64>::zero(3, 4));

    let matrix = Matrix(vec![vec![8, 1], vec![9, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
    println!("{:?}", matrix + matrix_2);

    let matrix = Matrix(vec![vec![1, 3], vec![2, 5]]);
    let matrix_2 = Matrix(vec![vec![3, 1], vec![1, 1]]);
    println!("{:?}", matrix - matrix_2);

    let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
    println!("{:?}", matrix - matrix_2);

    let matrix = Matrix(vec![vec![1, 3], vec![9, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
    println!("{:?}", matrix + matrix_2);
}
