use std::sync::atomic::{AtomicU64, Ordering};
use std::marker::PhantomData;

pub trait IDType {
    fn from_u64(value: u64) -> Self;
}

pub struct IDGenerator<T> {
    counter: AtomicU64,
    _marker: PhantomData<T>
}

impl<T: IDType> IDGenerator<T> {
    pub const fn new() -> Self {
        Self { 
            counter: AtomicU64::new(1),
            _marker: PhantomData, 
        }
    } 
    pub fn next(&self) -> T {
        let value = self.counter.fetch_add(1, Ordering::Relaxed);

        T::from_u64(value)
    }
}

