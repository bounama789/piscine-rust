use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    IV,
    V,
    IX,
    X,
    XL,
    L,
    XC,
    C,
    CD,
    D,
    CM,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => Nulla,
            1 => I,
            4 => IV,
            5 => V,
            9 => IX,
            10 => X,
            50 => XL,
            50 => L,
            90 => XC,
            100 => C,
            400 => CD,
            500 => D,
            900 => CM,
            1000 => M,
            _ => panic!("Invalid number"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(num: u32) -> Self {
        if num == 0 {
           return  RomanNumber(vec![Nulla]);
        }
        let mut num = num;
        let mut roman_number = vec![];
        let digits = vec![M, CM, D, CD, C, XC, L, XL, X, IX, V, IV, I];
        let values = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

        for (i, &value) in values.iter().enumerate() {
            while num >= value {
                num -= value;
                match digits[i] {
                    IV => {
                        roman_number.push(I);
                        roman_number.push(V);
                    },
                    IX => {
                        roman_number.push(I);
                        roman_number.push(X);
                    },
                    XL => {
                        roman_number.push(X);
                        roman_number.push(L);
                    },
                    XC => {
                        roman_number.push(X);
                        roman_number.push(C);
                    },
                    CD => {
                        roman_number.push(C);
                        roman_number.push(D);
                    },
                    CM => {
                        roman_number.push(C);
                        roman_number.push(M);
                    },
                    _=> roman_number.push(digits[i])
                }
                
                
            }
        }

        RomanNumber(roman_number)
    }
}
impl RomanNumber {
    pub fn to_decimal(&self) -> u32 {
        let mut total = 0;
        let mut previous_value = 0;

        for digit in self.0.iter().rev() {
            let value = match digit {
                Nulla => 0,
                I => 1,
                IV => 4,
                V => 5,
                IX => 9,
                X => 10,
                XL => 40,
                L => 50,
                XC => 90,
                C => 100,
                CD => 400,
                D => 500,
                CM => 900,
                M => 1000,
            };

            // If a smaller numeral comes before a larger numeral, we need to subtract
            if value < previous_value {
                total -= value;
            } else {
                total += value;
            }

            previous_value = value;
        }

        total
    }
}