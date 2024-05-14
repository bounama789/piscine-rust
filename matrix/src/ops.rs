use crate::{Matrix, Scalar};
use std::ops::{Add, Sub};

impl<T: Scalar> Add for Matrix<T> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.0.len();
        let c = if r > 0 {
            self.0.first().unwrap().len()
        } else {
            0
        };
        let mut m = Matrix::zero(r, c);
        for i in 0..r {
            for j in 0..c {
                m.0.get_mut(index)
            }
        };
    }
}

impl<T: Scalar> Sub for Matrix<T> {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
