pub trait DurationExt {
    fn increment(&mut self);
    fn decrement(&mut self) -> bool;
    fn to_string(&self) -> String;
}

impl DurationExt for std::time::Duration {
    fn increment(&mut self) {
        if let Some(duration) = self.checked_add(std::time::Duration::from_secs(1)) {
            *self = duration
        }
    }
    fn decrement(&mut self) -> bool {
        let left = self.checked_sub(std::time::Duration::from_secs(1));
        match left {
            None => false,
            Some(left) => {
                *self = left;
                true
            }
        }
    }
    fn to_string(&self) -> String {
        let seconds = self.as_secs();
        if seconds <= 60 {
            return format!("{seconds}s");
        }
        let (minutes, seconds_left) = (seconds / 60, seconds % 60);
        format!("{minutes}:{seconds_left:02}")
    }
}
