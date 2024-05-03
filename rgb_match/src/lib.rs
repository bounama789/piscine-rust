#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match (first, second) {
            (f, s) if f == self.r && s == self.g => {
                self.r = self.g;
                self.g = f;
            }
            (f, s) if f == self.r && s == self.b => {
                self.r = self.b;
                self.b = f;
            }
            (f, s) if f == self.r && s == self.a => {
                self.r = self.a;
                self.a = f;
            }
            (f, s) if f == self.g && s == self.r => {
                self.g = self.r;
                self.r = f;
            }
            (f, s) if f == self.g && s == self.b => {
                self.g = self.b;
                self.b = f;
            }
            (f, s) if f == self.g && s == self.a => {
                self.g = self.a;
                self.a = f;
            }
            (f, s) if f == self.b && s == self.r => {
                self.b = self.r;
                self.r = f;
            }
            (f, s) if f == self.b && s == self.g => {
                self.b = self.g;
                self.g = f;
            }
            (f, s) if f == self.b && s == self.a => {
                self.b = self.a;
                self.a = f;
            }
            (f, s) if f == self.a && s == self.r => {
                self.a = self.r;
                self.r = f;
            }
            (f, s) if f == self.a && s == self.b => {
                self.a = self.b;
                self.b = f;
            }
            (f, s) if f == self.a && s == self.g => {
                self.a = self.g;
                self.g = f;
            }
            _ => {}
        }
        self
    }
}
