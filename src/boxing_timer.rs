use crate::bell::Bell;
use crate::duration::Duration;
use crate::errors::BoxingTimerErrorKind;
use crate::sequence::Sequence;
use crate::status::Status;
use crate::stopwatch::Stopwatch;
pub const DEFAULT_INTERVAL: u64 = 1000;

#[derive(Debug, Default)]
pub struct BoxingTimer {
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

impl BoxingTimer {
    pub fn new(
        prepare: u64,
        fight: u64,
        rest: u64,
        rounds: usize,
    ) -> Result<Self, BoxingTimerErrorKind> {
        let sequence = Sequence::boxing_sequence(
            rounds,
            Duration::from_secs(prepare),
            Duration::from_secs(fight),
            Duration::from_secs(rest),
        )?;
        let stopwatch = Stopwatch::new(Duration::default());
        Ok(Self {
            stopwatch,
            sequence,
            ..Self::default()
        })
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
    pub fn reset_beginning(&mut self) {
        log::info!("reseting beginning");
        if !self.sequence.is_empty() {
            self.current_item = Some(0);
            self.stopwatch.set(*self.sequence[0].duration());
        }
    }
    pub fn reset_current(&mut self) {
        log::info!("reseting current");
        if let Some(current_item) = self.current_item {
            self.stopwatch.set(*self.sequence[current_item].duration());
        }
    }
    pub fn tick(&mut self) {
        let Some(current_item) = self.current_item else {
            return;
        };
        if self.status == Status::Paused {
            return;
        }
        if self.stopwatch.decrement() {
            return;
        }
        Bell::ring();
        if current_item == self.sequence.len() - 1 {
            self.status = Status::Paused;
            self.current_item = None;
            return;
        }
        self.current_item = Some(current_item + 1);
        self.stopwatch
            .set(*self.sequence[current_item + 1].duration())
    }
    pub fn goto_previous(&mut self) {
        log::info!("goto previous");
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
        log::info!("goto next");
        if let Some(current_item) = self.current_item {
            if current_item < self.sequence.len() - 1 {
                self.current_item = Some(current_item + 1);
                self.stopwatch
                    .set(*self.sequence[current_item + 1].duration());
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
