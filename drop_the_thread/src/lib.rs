use std::{
    cell::{Cell, RefCell},
    process,
};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let index = self.track_worker();
        let thread = Thread::new_thread(index, c, self);

        self.states.borrow_mut().push(false);
        (index, thread)
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        *self.states.borrow().get(id).unwrap_or(&false)
    }

    pub fn add_drop(&self, id: usize) {
        if let Some(state) = self.states.borrow().get(id) {
            match state {
                true => {
                    panic!("{} is already dropped", id);
                }
                false => {
                    let mut states = self.states.borrow_mut();
                    *states.get_mut(id).unwrap() = true;
                    let a = self.drops.take();
                    self.drops.set(a + 1);
                }
            }
        } else {
            println!("Invalid ID: {}", id);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmb: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Thread {
            pid: p,
            cmb: c,
            parent: t,
        }
    }
    pub fn skill(self) {
        drop(self)
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid)
    }
}
