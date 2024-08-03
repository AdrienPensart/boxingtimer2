use crate::bell::Bell;
use crate::duration::Duration;
use crate::sequence::Sequence;
use crate::status::Status;
use crate::stopwatch::Stopwatch;
use gloo::console::log;
pub const DEFAULT_INTERVAL: u64 = 1000;

#[derive(Debug, Default, Clone)]
pub struct Timer {
    /// Is the timer running ?
    status: Status,
    /// Current state of training
    // state: State,
    /// Left seconds in current round
    stopwatch: Stopwatch,
    /// Current exercise
    current_item: Option<usize>,
    /// Full sequence
    sequence: Sequence,
}

impl Timer {
    pub fn new(sequence: Sequence) -> Self {
        let stopwatch = Stopwatch::new(Duration::default());
        Self {
            stopwatch,
            sequence,
            ..Self::default()
        }
    }
    pub fn current_item(&self) -> Option<usize> {
        self.current_item
    }
    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }
    pub fn stopwatch(&self) -> Stopwatch {
        self.stopwatch
    }
    pub fn set_sequence(&mut self, sequence: &Sequence) {
        log!("set new sequence");
        self.sequence = sequence.clone();
        self.reset_beginning()
    }
    pub fn reset_beginning(&mut self) {
        log!("reseting beginning");
        if !self.sequence.is_empty() {
            self.current_item = Some(0);
            self.stopwatch.set(*self.sequence[0].duration());
        } else {
            self.current_item = None;
        }
    }
    pub fn reset_current(&mut self) {
        log!("reseting current");
        if let Some(current_item) = self.current_item {
            self.stopwatch.set(*self.sequence[current_item].duration());
        }
    }
    pub fn tick(&mut self, bell: &Bell) {
        let Some(current_item) = self.current_item else {
            return;
        };
        if self.status == Status::Paused {
            return;
        }
        if self.stopwatch.decrement() {
            return;
        }
        bell.ring();
        if current_item == self.sequence.len() - 1 {
            if self.sequence.restart() {
                self.current_item = Some(0);
                self.stopwatch.set(*self.sequence[0].duration());
            } else {
                self.status = Status::Paused;
                self.current_item = None;
            }
        } else {
            self.current_item = Some(current_item + 1);
            self.stopwatch
                .set(*self.sequence[current_item + 1].duration())
        }
    }
    pub fn goto_previous(&mut self) {
        log!("goto previous");
        if let Some(current_item) = self.current_item {
            let next_item = if current_item > 0 {
                current_item - 1
            } else {
                0
            };
            self.current_item = Some(next_item);
            self.stopwatch.set(*self.sequence[next_item].duration());
        } else if self.current_item.is_none() && !self.sequence.is_empty() {
            self.current_item = Some(0);
            self.stopwatch.set(*self.sequence[0].duration())
        }
    }
    pub fn goto_next(&mut self) {
        log!("goto next");
        if let Some(current_item) = self.current_item {
            if current_item < self.sequence.len() - 1 {
                self.current_item = Some(current_item + 1);
                self.stopwatch
                    .set(*self.sequence[current_item + 1].duration());
            } else if self.sequence().restart() {
                self.current_item = Some(0);
                self.stopwatch.set(*self.sequence[0].duration());
            }
        } else if self.current_item.is_none() && !self.sequence.is_empty() {
            self.current_item = Some(0);
            self.stopwatch.set(*self.sequence[0].duration())
        }
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn toggle(&mut self) {
        self.status.toggle();
        if self.status == Status::Running
            && self.current_item.is_none()
            && !self.sequence.is_empty()
        {
            self.current_item = Some(0);
            self.stopwatch.set(*self.sequence[0].duration())
        }
    }
    pub fn next_status(&self) -> Status {
        self.status.next()
    }
}
