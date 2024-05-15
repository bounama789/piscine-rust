use crate::{Matrix, Scalar};
use std::ops::{Add, Sub};

impl<T: Scalar> Add for Matrix<T> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let len = self.0.len();
        let mut m = Vec::new();
        for i in 0..len {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
            let mut r = Vec::new();
            for j in 0..self.0[i].len() {
                let n = self.0[i][j].add(rhs.0[i][j]);
                r.push(n)
            }
            m.push(r);
        }
        Some(Matrix(m))
    }
}

impl<T: Scalar> Sub for Matrix<T> {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let len = self.0.len();
        let mut m = Vec::new();
        for i in 0..len {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
            let mut r = Vec::new();
            for j in 0..self.0[i].len() {
                let n = self.0[i][j].sub(rhs.0[i][j]);
                r.push(n)
            }
            m.push(r);
        }
        Some(Matrix(m))
    }
}
