use hashbrown::HashMap;

pub struct Registry<T> {
    pub reg: HashMap<String, T>,
}

impl<T: Clone> Registry<T> {
    pub fn new() -> Self {
        Self {
            reg: HashMap::new(),
        }
    }

    pub fn register(&mut self, str: String, value: T) {
        self.reg.insert(str, value);
    }

    pub fn get(&self, str: &String) -> Option<T> {
        let val = self.reg.get(str);

        if val.is_none() {
            return None;
        }

        return Some(val.unwrap().clone());
    }
}
