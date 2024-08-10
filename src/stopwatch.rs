use crate::duration::DurationExt;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Stopwatch {
    duration: std::time::Duration,
    left: std::time::Duration,
    elapsed: std::time::Duration,
}

impl Stopwatch {
    pub fn set(&mut self, duration: &std::time::Duration) {
        self.duration = *duration;
        self.left = *duration;
    }
    pub fn reset(&mut self) {
        self.left = self.duration;
    }
    pub fn decrement(&mut self) -> bool {
        let success = self.left.decrement();
        if success {
            self.elapsed.increment();
        }
        success
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

// impl From<&u64> for Stopwatch {
//     fn from(seconds: &u64) -> Self {
//         Self {
//             duration: Duration::from_secs(*seconds),
//             left: Duration::from_secs(*seconds),
//             ..Self::default()
//         }
//     }
// }

// impl From<u64> for Stopwatch {
//     fn from(seconds: u64) -> Self {
//         Self {
//             duration: Duration::from_secs(seconds),
//             left: Duration::from_secs(seconds),
//             ..Self::default()
//         }
//     }
// }

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
