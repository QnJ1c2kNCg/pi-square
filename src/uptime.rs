use std::time::{Duration, Instant};

pub struct Uptime {
    start: Instant,
}

impl Uptime {
    pub fn new() -> Self {
        Uptime {
            start: Instant::now(),
        }
    }

    pub fn get(&self) -> Duration {
        self.start.elapsed()
    }
}
