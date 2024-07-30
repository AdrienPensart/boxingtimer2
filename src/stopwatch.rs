use crate::duration::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct Stopwatch {
    duration: Duration,
    left: Duration,
    elapsed: Duration,
}

impl Stopwatch {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            left: duration,
            elapsed: Duration::default(),
        }
    }
    pub fn set(&mut self, duration: Duration) {
        self.duration = duration;
        self.left = duration;
    }
    pub fn decrement(&mut self) -> bool {
        let success = self.left.decrement();
        if success {
            self.elapsed.increment();
        }
        success
    }
    pub fn duration(&self) -> &Duration {
        &self.duration
    }
    pub fn left(&self) -> &Duration {
        &self.left
    }
    pub fn elapsed(&self) -> &Duration {
        &self.elapsed
    }
}

impl std::fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} / {}", self.left, self.duration)
    }
}
