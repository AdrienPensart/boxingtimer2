#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash)]
pub struct Duration(std::time::Duration);

impl Duration {
    pub fn from_secs(seconds: u64) -> Self {
        Self(std::time::Duration::from_secs(seconds))
    }
    pub fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }
    pub fn increment(&mut self) {
        if let Some(duration) = self.0.checked_add(std::time::Duration::from_secs(1)) {
            self.0 = duration
        }
    }
    pub fn decrement(&mut self) -> bool {
        let left = self.0.checked_sub(std::time::Duration::from_secs(1));
        match left {
            None => {
                self.0 = std::time::Duration::from_secs(0);
                false
            }
            Some(left) => {
                self.0 = left;
                true
            }
        }
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seconds = self.0.as_secs();
        let (minutes, seconds_left) = (seconds / 60, seconds % 60);
        write!(f, "{minutes}:{seconds_left:02}")
        // let duration = duration_string::DurationString::from(self.0);
        // write!(f, "{duration}")
    }
}
