#[derive(PartialEq, Debug)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));
pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((a, b), (c, d)) = m;
    Matrix((a, c), (b, d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let matrix = Matrix((1, 3), (4, 5));
        let result = transpose(matrix);
        assert_eq!(result, Matrix((1, 4), (3, 5)));
    }
}
