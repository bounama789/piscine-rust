use std::ops::{Add, Mul};

pub trait Scalar: Add<Output = Self> + Mul<Output = Self> + Sized + Clone + PartialEq {}

impl<S> Scalar for S where S: Add<Output = Self> + Mul<Output = Self> + Clone + Sized + PartialEq {}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut result = Vec::new();
        for i in 0..self.0.len() {
            result.push(self.0[i].clone() + other.0[i].clone())
        }
        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new(v: Vec<T>) -> Self {
        Vector(v)
    }
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut result = self.0[0].clone() * other.0[0].clone();
        for i in 1..self.0.len() {
            result = result + self.0[i].clone() * other.0[i].clone();
        }
        Some(result)
    }
}
