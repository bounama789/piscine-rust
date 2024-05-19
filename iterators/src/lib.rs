#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
    first: bool,
}

impl Collatz {
    pub fn new(v: u64) -> Self {
        Collatz { v, first: true }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            return None;
        }
        if self.first {
            self.first = false;
            return Some(self.v);
        }
        if self.v == 1 {
            None
        } else {
            self.v = if self.v % 2 == 0 {
                self.v / 2
            } else {
                3 * self.v + 1
            };
            Some(self.v)
        }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut collatz = Collatz::new(n);
    let mut steps = 0;
    while let Some(_) = collatz.next() {
        steps += 1;
    }
    steps - 1 // subtracting 1 to exclude the initial number itself
}

fn main() {
    for val in Collatz::new(10) {
        println!("{}", val); // This will print the sequence starting with 10
    }
    println!("{:?}", collatz(0));  // Special case, should handle this gracefully
    println!("{:?}", collatz(1));  // 0 steps since it's already 1
    println!("{:?}", collatz(4));  // 2 steps: 4 -> 2 -> 1
    println!("{:?}", collatz(5));  // 5 steps: 5 -> 16 -> 8 -> 4 -> 2 -> 1
    println!("{:?}", collatz(6));  // 8 steps: 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
    println!("{:?}", collatz(7));  // 16 steps: 7 -> 22 -> 11 -> 34 -> 17 -> 52 -> 26 -> 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
    println!("{:?}", collatz(12)); // 9 steps: 12 -> 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
}
