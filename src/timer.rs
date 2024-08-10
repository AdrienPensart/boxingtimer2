use crate::bell::Bell;
use crate::duration::Duration;
use crate::sequence::Sequence;
use crate::status::Status;
use derive_more::{Deref, DerefMut, IntoIterator};
use gloo::console::log;
pub const DEFAULT_INTERVAL: u64 = 1000;

#[derive(Debug, Default, Clone, Deref, DerefMut, IntoIterator)]
pub struct Timer {
    /// Is the timer running ?
    status: Status,
    /// Full sequence
    #[deref]
    #[deref_mut]
    #[into_iterator]
    sequence: Sequence,
    /// Elasped time running
    elapsed: Duration,
}

impl Timer {
    pub fn new(sequence: &Sequence) -> Self {
        Self {
            sequence: sequence.clone(),
            ..Default::default()
        }
    }
    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }
    pub fn set_sequence(&mut self, sequence: &Sequence) {
        self.sequence = sequence.clone();
    }
    pub fn tick(&mut self, bell: &Bell) {
        if self.status.paused() {
            log!("paused, skipping tick");
            return;
        }

        self.elapsed.increment();

        if !self.decrement() {
            log!("goto next");
            self.goto_next(false);
            bell.ring();
            if self.ended() {
                if self.cycle() {
                    self.reset_beginning(false);
                } else {
                    self.status.toggle();
                }
            }
        } else {
            log!("decremented");
        }
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn toggle(&mut self) {
        self.status = self.next_status().clone();
    }
    pub fn next_status(&self) -> &Status {
        self.status.next()
    }
    pub fn paused(&self) -> bool {
        self.status.paused()
    }
    pub fn elapsed(&self) -> &Duration {
        &self.elapsed
    }
}
