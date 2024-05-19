use roman_numbers_iter::{RomanNumber, RomanNumberExt as _};

fn main() {
	let mut number = RomanNumber::from(900);

	println!("{:?}", number);
	println!("{:?}", number.next());
}