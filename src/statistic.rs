use std::{
    fmt::{self, Debug},
    ops::AddAssign,
    time::{Duration, Instant},
};

#[derive(Default)]
pub struct Average {
    sum: f64,
    num: usize,
}

impl Debug for Average {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.sum / self.num as f64)
    }
}

impl AddAssign<usize> for Average {
    fn add_assign(&mut self, rhs: usize) {
        self.sum += rhs as f64;
        self.num += 1;
    }
}

impl AddAssign<f64> for Average {
    fn add_assign(&mut self, rhs: f64) {
        self.sum += rhs;
        self.num += 1;
    }
}

#[derive(Default)]
pub struct AverageDuration {
    sum: Duration,
    num: usize,
}

impl Debug for AverageDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.num == 0 {
            write!(f, "None")
        } else {
            write!(f, "{:?}", self.sum / self.num as u32)
        }
    }
}

impl AddAssign<Duration> for AverageDuration {
    fn add_assign(&mut self, rhs: Duration) {
        self.sum += rhs;
        self.num += 1;
    }
}

#[derive(Default)]
pub struct SuccessRate {
    succ: usize,
    fail: usize,
}

impl SuccessRate {
    pub fn success(&mut self) {
        self.succ += 1;
    }

    pub fn fail(&mut self) {
        self.fail += 1;
    }

    pub fn statistic(&mut self, success: bool) {
        if success {
            self.success()
        } else {
            self.fail()
        }
    }
}

impl Debug for SuccessRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "success: {}, fail: {}, success rate: {:.2}%",
            self.succ,
            self.fail,
            (self.succ as f64 / (self.succ + self.fail) as f64) * 100_f64
        )
    }
}

#[derive(Default)]
pub struct Case(String);

impl Case {
    pub fn new<S: ToString>(s: S) -> Self {
        Self(s.to_string())
    }
}

impl Debug for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct RunningTime {
    start: Instant,
}

impl Default for RunningTime {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl Debug for RunningTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}s", self.start.elapsed().as_secs_f64())
    }
}
