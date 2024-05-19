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