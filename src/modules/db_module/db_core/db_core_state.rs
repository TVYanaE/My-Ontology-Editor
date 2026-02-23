pub enum DBCoreState {
    Ready, 
    Shutdown,
    Processing,
}

impl Default for DBCoreState {
    fn default() -> Self {
        Self::Ready
    }
}
