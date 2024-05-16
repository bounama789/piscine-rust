use lalgebra_scalar::Scalar;


#[derive(Debug,PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![Vec::new()])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut res = Vec::new();

        for _ in 0..row  {
            let mut tmp = Vec::new();
            for _ in 0..col {
                tmp.push(T::zero());
            }
            res.push(tmp);
        };

        Matrix(res)
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut m = Matrix::zero(n, n);
        let mut i = n;
        m.0.iter_mut().for_each(|row|{
            row[n-i] = T::one();
            i-=1;
        });
        m 
	}
}