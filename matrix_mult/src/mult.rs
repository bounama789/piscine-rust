use std::ops::Mul;

use lalgebra_scalar::Scalar;

use crate::Matrix;

impl<T: Scalar> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        let a = self.0.last();

        if let Some(e) = a {
            return e.len();
        }
        0
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut col = Vec::new();

        for i in 0..self.0.len() {
            col.push(self.0[i][n])
        }
        col
    }
}

impl<T: Scalar> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let len = self.0.len();
        let mut m = Vec::new();
        for i in 0..len {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
            let mut r = Vec::new();
            for j in 0..self.0[i].len() {
                let sum = self
                    .row(i)
                    .iter()
                    .zip(rhs.col(j).iter())
                    .fold(T::default(), |acc, (a, b)| acc + a.clone() * b.clone());
                r.push(sum);
            }
            m.push(r);
        }
        Some(Matrix(m))
    }
}
