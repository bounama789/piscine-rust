pub use std::{cell::RefCell, rc::Rc};

// Define the Logger trait
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

// Define the Tracker structure
pub struct Tracker<'z, L: Logger> {
    logger: &'z L,
    pub value: RefCell<usize>,
    max: u32,
}
impl<'z, L: Logger> Tracker<'z, L> {
    pub fn new(logger: &'z L, max: u32) -> Self {
        Tracker {
            logger,
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        *self.value.borrow_mut() = Rc::strong_count(value);
        let percentage = (*self.value.borrow() * 100) as u32 / self.max as u32;
        if percentage >= 100 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70 {
            self.logger.warning(&format!(
                "you have used up over {}% of your quota! Proceeds with precaution",
                percentage
            ));
        }
    }

    pub fn peek(&self, value: &Rc<usize>) {
        let percentage = (Rc::strong_count(value) * 100) as u32 / self.max;
        self.logger.info(&format!(
            "you are using up to {}% of your quota",
            percentage
        ));
    }
}
