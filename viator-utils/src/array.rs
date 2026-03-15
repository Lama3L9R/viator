use std::mem::MaybeUninit;

trait ArrayLifeHacks {
    
}

pub trait ArrayUnwrapMany <T, const N: usize> {
    fn unwrap_many(self) -> anyhow::Result<[T; N]>;
}

impl <T, const N: usize> ArrayUnwrapMany<T, N> for [Option<T>; N] {
    fn unwrap_many(self) -> anyhow::Result<[T; N]> {
        
        if self.iter().any(|it| it.is_none()) {
            return Err(anyhow::anyhow!("At least one of the given element is None"))
        } else {
            return Ok(self.map(|it| it.unwrap()))
        }
    }
}
