use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct MaybeValue <T> {
    pub value: Option<T>
}

impl <T> Deref for MaybeValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return self.value.as_ref().unwrap();
    }
}

impl <T> DerefMut for MaybeValue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return self.value.as_mut().unwrap();
    }
}


#[macro_export]
macro_rules! maybe {
    (null) => {
        viator_utils::maybe_value::MaybeValue { value: None }
    };

    ($val: expr) => {
        viator_utils::maybe_value::MaybeValue { value: Some($val) }
    };

    () => {
        viator_utils::maybe_value::MaybeValue { value: None }
    };
 }