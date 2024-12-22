use crate::defaults;
use crate::sequence::Sequence;
use crate::status::Status;
use crate::stopwatch::Stopwatch;
use dioxus::logger::tracing::info;

#[derive(Default, Debug)]
pub struct MobileTimer {
    status: Status,
    sequence: Sequence,
    preparation: Stopwatch,
    changed: bool,
}

impl MobileTimer {
    pub fn from_sequence(sequence: &Sequence) -> Self {
        Self::new(defaults::PREPARE_DURATION, sequence)
    }
    pub fn new(preparation: std::time::Duration, sequence: &Sequence) -> Self {
        Self {
            preparation: Stopwatch::from(preparation),
            sequence: sequence.clone(),
            ..Default::default()
        }
    }
    pub fn left(&self) -> &std::time::Duration {
        if let Some(item) = self.sequence.get() {
            return item.left();
        }
        self.preparation.left()
    }
    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }
    pub fn always_ring(&self) {
        self.sequence.signal().always_ring();
    }
    pub fn restart_sequence(&mut self) {
        self.preparation.reset();
        self.changed = true;
        self.sequence.reset();
    }
    pub fn restart_item(&mut self) {
        self.preparation.reset();
        self.sequence.reset_current();
    }
    pub fn tick(&mut self) -> bool {
        // info!(
        //     "tick : {} / {} / {}",
        //     self.status, self.changed, self.preparation
        // );
        if self.changed {
            self.changed = false;
            return false;
        }
        if self.status.paused() {
            return false;
        }

        if self.sequence.get().is_none() && self.preparation.decrement() {
            // info!("preparation : {}", self.preparation);
            if self.sequence.signal().sound().is_beep() && self.preparation.last_seconds() {
                self.sequence.signal().ring();
            }
            return false;
        }
        self.preparation.reset();

        if self.sequence.decrement() {
            if self.sequence.signal().sound().is_beep() && self.sequence.last_seconds() {
                self.sequence.signal().ring();
            }
            return false;
        }

        if !self.sequence.is_empty() && self.sequence.signal().sound().is_bell() {
            self.sequence.signal().ring();
        }
        info!("goto next");
        if self.sequence.auto_next().is_some() {
            return true;
        }
        self.status.toggle();
        false
    }
    pub fn manual_next(&mut self) {
        if self.running() {
            self.changed = true;
        }
        self.sequence.manual_next();
    }
    pub fn manual_previous(&mut self) {
        if self.running() {
            self.changed = true;
        }
        self.sequence.goto_previous();
    }
    pub fn label(&self) -> &str {
        if self.sequence.get().is_none() {
            return defaults::PREPARE_LABEL;
        }
        let Some(item) = self.sequence.get() else {
            return defaults::PREPARE_LABEL;
        };
        item.name()
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn shuffle(&mut self) {
        self.sequence.shuffle()
    }
    pub fn toggle(&mut self) {
        info!("toggle");
        self.status = self.next_status().clone();
        if self.running() {
            self.changed = true;
        }
    }
    pub fn next_status(&self) -> &Status {
        self.status.next()
    }
    pub fn paused(&self) -> bool {
        self.status.paused()
    }
    pub fn running(&self) -> bool {
        self.status.running()
    }
}
