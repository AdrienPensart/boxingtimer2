use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DurationSeconds;

use crate::duration::DurationExt;

#[serde_as]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Stopwatch {
    #[serde_as(as = "DurationSeconds<u64>")]
    duration: std::time::Duration,
    #[serde(skip)]
    left: std::time::Duration,
    #[serde(skip)]
    elapsed: std::time::Duration,
}

impl Stopwatch {
    pub fn reset(&mut self) -> &mut Self {
        self.left = self.duration;
        self
    }
    pub fn increment(&mut self) {
        self.left.increment();
    }
    pub fn decrement(&mut self) -> bool {
        let success = self.left.decrement();
        if success {
            self.elapsed.increment();
        }
        success
    }
    pub fn last_seconds(&self) -> bool {
        self.left.as_secs() < 3
    }
    pub fn duration(&self) -> &std::time::Duration {
        &self.duration
    }
    pub fn left(&self) -> &std::time::Duration {
        &self.left
    }
    pub fn elapsed(&self) -> &std::time::Duration {
        &self.elapsed
    }
}

impl From<&std::time::Duration> for Stopwatch {
    fn from(duration: &std::time::Duration) -> Self {
        Self {
            duration: *duration,
            left: *duration,
            ..Self::default()
        }
    }
}

impl From<std::time::Duration> for Stopwatch {
    fn from(duration: std::time::Duration) -> Self {
        Self {
            duration,
            left: duration,
            ..Self::default()
        }
    }
}

impl std::fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} / {}",
            self.left.to_string(),
            self.duration.to_string()
        )
    }
}
