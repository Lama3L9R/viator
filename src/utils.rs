use std::ops::Deref;
use std::rc::Rc;

pub struct RBox<T>(Rc<Box<T>>);

impl <T> RBox<T> {
    pub fn new(data: T) -> Self {
        return RBox(Rc::new(Box::new(data)));
    }
}

impl <T> Deref for RBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return self.0.deref();
    }
}
impl <T> Clone for RBox<T> {
    fn clone(&self) -> Self {
        return RBox(self.0.clone());
    }
}