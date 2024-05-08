pub mod messenger;

pub use messenger::{Logger, Tracker};
pub use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: usize) -> Self {
        Worker {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        let (key, value) = msg.split_once(": ").unwrap();
        self.mapped_messages
            .borrow_mut()
            .insert(key.to_string(), value.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("{key}: {value}"));
    }

    fn info(&self, msg: &str) {
        let (key, value) = msg.split_once(": ").unwrap();
        self.mapped_messages
            .borrow_mut()
            .insert(key.to_string(), value.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("{key}: {value}"));
    }

    fn error(&self, msg: &str) {
        let (key, value) = msg.split_once(": ").unwrap();
        self.mapped_messages
            .borrow_mut()
            .insert(key.to_string(), value.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("{key}: {value}"));
    }
}
