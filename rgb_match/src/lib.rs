#[derive(Debug,Clone, Copy)]

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        self.r = if self.r == first { second } else { self.r };
        self.g = if self.g == first { second } else { self.g };
        self.b = if self.b == first { second } else { self.b };
        self.a = if self.a == first { second } else { self.a };
        self
    }
}
