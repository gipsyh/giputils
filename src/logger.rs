use log::{Level, log};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub struct IntervalLogger {
    last: Instant,
    interval: Duration,
}

impl IntervalLogger {
    #[inline]
    pub fn new(interval: Duration) -> Self {
        Self {
            last: Instant::now(),
            interval,
        }
    }

    #[inline]
    pub fn log<T: Display>(&mut self, level: Level, msg: T) {
        let now = Instant::now();
        if now.duration_since(self.last) >= self.interval {
            log!(level, "{msg}");
            self.last = now;
        }
    }
}

impl Default for IntervalLogger {
    #[inline]
    fn default() -> Self {
        Self::new(Duration::from_secs(10))
    }
}
