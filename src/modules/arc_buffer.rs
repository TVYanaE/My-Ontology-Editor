use std::{
    sync::{Arc, Mutex},
    clone::Clone,
};

use arc_swap::ArcSwap;

pub type Snapshot<T> = Arc<T>;

pub struct ArcBuffer<T> 
where 
    T: Clone
{
    write_buffer: Mutex<T>,
    read_buffer: ArcSwap<T>,
}

impl<T> ArcBuffer<T> 
where 
    T: Clone
{
    pub fn new(value: T) -> Self {
        Self {
            write_buffer: Mutex::new(value.clone()),
            read_buffer: ArcSwap::from_pointee(value),
        }
    }

    pub fn push(&self, value: T) {
        *self.write_buffer.lock().unwrap() = value;
    }

    pub fn swap(&self) {
        let state = self.write_buffer.lock().unwrap();

        let snapshot = Arc::new(state.clone());

        self.read_buffer.store(snapshot);
    }

    pub fn get_read_buffer(&self) -> Snapshot<T> {
        self.read_buffer.load_full()
    }
}
