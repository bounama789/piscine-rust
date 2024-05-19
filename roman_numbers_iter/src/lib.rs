extern crate roman_numbers;

pub use roman_numbers::{RomanDigit, RomanNumber};

pub struct RomanNumberIter {
    current: Option<RomanNumber>,
    decimal:u32
}

impl RomanNumberIter {
    pub fn new(start: RomanNumber) -> Self {
        RomanNumberIter {
            current: Some(start.clone()),
            decimal: start.to_decimal()
        }
    }

    pub fn next(&mut self) -> Option<RomanNumber> {
        if let Some(_) = self.current.take() {
            Some(RomanNumber::from(self.decimal+1))
        } else {
            None
        }
    }
}

pub trait RomanNumberExt {
    fn next(&mut self) -> Option<RomanNumber>;
}

impl RomanNumberExt for RomanNumber {
    fn next(&mut self) -> Option<RomanNumber> {
        // Create an iterator with self as the starting value
        let mut iter = RomanNumberIter::new(self.clone());
        iter.next()
    }
}



