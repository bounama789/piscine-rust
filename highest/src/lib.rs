#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if let Some(e) = self.numbers.last() {
            return Some(*e)
        }
        None
    }

    pub fn highest(&self) -> Option<u32> {
        if let Some(e) = self.numbers.iter().max() {
            return Some(*e)
        }
        None
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut nbrs = self.numbers.to_vec();
        nbrs.sort();
        let x = nbrs.len()-3;

        if let Some(three) = nbrs.get_mut(x..) {
            three.reverse();
            return three.to_vec();
        }
        self.numbers.to_vec()
    }
}