use std::boxed::Box;
use std::rc::Rc;

use hashbrown::HashMap;

pub struct Registry<T> {
    pub reg: HashMap<String, Rc<Box<T>>>,
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Self {
            reg: HashMap::new(),
        }
    }

    pub fn register(&mut self, str: String, value: T) {
        self.reg.insert(str, Rc::new(Box::new(value)));
    }

    pub fn get(&self, str: &String) -> Option<Rc<Box<T>>> {
        let val = self.reg.get(str);

        if val.is_none() {
            return None;
        }

        return Some(val.unwrap().clone());
    }
}
