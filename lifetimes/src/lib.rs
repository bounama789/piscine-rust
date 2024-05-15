#[derive(Debug,Clone, Copy)]
pub struct Person<'s> {
    pub name: &'s str,
    pub age: u8,
}

impl<'s> Person<'s> {
    pub fn new(name: &str) -> Person {
        Person { name, age: 0 }
    }
}
