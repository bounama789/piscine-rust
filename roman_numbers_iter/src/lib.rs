extern crate roman_numbers;

pub use roman_numbers::{RomanDigit, RomanNumber};

pub struct RomanNumberIter {
    decimal:u32
}

impl RomanNumberIter {
    pub fn new(start: RomanNumber) -> Self {
        RomanNumberIter {
            decimal: start.to_decimal()
        }
    }

    pub fn next(&mut self) -> RomanNumber {
           RomanNumber::from(self.decimal+1)
    }
}

pub trait RomanNumberExt {
    fn next(&mut self) -> Option<RomanNumber>;
}

impl RomanNumberExt for RomanNumber {
    fn next(&mut self) -> Option<RomanNumber> {
        // Create an iterator with self as the starting value
        let mut iter = RomanNumberIter::new(self.clone());
        Some( iter.next())
       
    }
}



