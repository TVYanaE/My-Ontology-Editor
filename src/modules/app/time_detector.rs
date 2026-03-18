use std::time::Instant;

pub struct TimeDetector {
    start_time_mes: Instant,
}

impl TimeDetector {
    pub fn new() -> Self {
        Self { 
            start_time_mes: Instant::now(),
        }
    }
    pub fn start_measurement(&mut self) {
        self.start_time_mes = Instant::now();
    }
    pub fn stop_and_display(&mut self) {
        let delt = Instant::now() - self.start_time_mes;
        println!("Measuremented Time: {:?}", delt);
    }
}
