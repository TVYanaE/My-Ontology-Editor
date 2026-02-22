use std::{
    mem::take,
    sync::{Arc, Mutex},
};

use arc_swap::ArcSwap;

pub type Snapshot<T> = Arc<Vec<T>>;

pub struct ArcBuffer<T> {
    write_buffer: Mutex<Vec<T>>,
    read_buffer: ArcSwap<Vec<T>>,
}

impl<T> ArcBuffer<T> {
    pub fn new() -> Self {
        Self {
            write_buffer: Mutex::new(Vec::new()),
            read_buffer: ArcSwap::from_pointee(Vec::new()),
        }
    }

    pub fn push(&self, value: T) {
        self.write_buffer.lock().unwrap().push(value);
    }

    pub fn swap(&self) {
        let mut write_buffer = self.write_buffer.lock().unwrap();

        let new_vec = take(&mut *write_buffer);

        let snapshot = Arc::new(new_vec);

        self.read_buffer.store(snapshot);
    }

    pub fn get_read_buffer(&self) -> Arc<Vec<T>> {
        self.read_buffer.load_full()
    }
}
