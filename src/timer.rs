use crate::bell::Bell;
use crate::duration::DurationExt;
use crate::sequence::Sequence;
use crate::status::Status;
use derive_more::{Deref, DerefMut};
use gloo::console::log;
pub const DEFAULT_INTERVAL: u64 = 1000;

#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Timer {
    /// Is the timer running ?
    status: Status,
    /// Full sequence
    #[deref]
    #[deref_mut]
    sequence: Box<Sequence>,
    /// Elasped time running
    elapsed: std::time::Duration,
}

impl Timer {
    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }
    pub fn set_sequence(&mut self, sequence: Box<Sequence>) {
        self.sequence = sequence;
    }
    pub fn tick(&mut self, bell: &Bell) {
        if self.status.paused() {
            log!("paused, skipping tick");
            return;
        }

        self.elapsed.increment();

        if !self.decrement() {
            if !self.sequence.is_empty() {
                bell.ring();
            }
            if self.ended() {
                if self.cycling() {
                    self.reset_beginning(false);
                } else {
                    self.status.toggle();
                }
                return;
            }
            log!("goto next");
            self.goto_next(false);
        } else {
            log!("decremented");
        }
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn toggle(&mut self) {
        if !self.sequence.is_empty() {
            self.status = self.next_status().clone();
        }
    }
    pub fn next_status(&self) -> &Status {
        self.status.next()
    }
    pub fn paused(&self) -> bool {
        self.status.paused()
    }
    pub fn elapsed(&self) -> &std::time::Duration {
        &self.elapsed
    }
}
