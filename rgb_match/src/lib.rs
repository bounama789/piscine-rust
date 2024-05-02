#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut x = first;
        let mut y = second;
        if self.r == x {
            self.r = y;
            (x, y) = (y, x);
        }

        if self.g == x {
            self.g = y;
            (x, y) = (y, x);
        }
        if self.b == x {
            self.b = y;
            (x, y) = (y, x);
        }
        if self.a == x {
            self.a = y;
        }
        self
    }
}
