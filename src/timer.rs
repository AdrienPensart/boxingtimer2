use crate::signal::SoundSignal;
use crate::status::Status;
use dioxus::logger::tracing::info;
use sport::defaults;
use sport::sequence::Sequence;
use sport::stopwatch::Stopwatch;

#[derive(Debug)]
pub struct Timer {
    status: Status,
    sequence: Sequence,
    preparation: Stopwatch,
    changed: bool,
    sound_signal: SoundSignal,
}

impl Timer {
    #[must_use]
    pub fn from_sequence(sequence: &Sequence, sound_signal: &SoundSignal) -> Self {
        Self::new(defaults::PREPARE_DURATION, sequence, sound_signal)
    }
    #[must_use]
    pub fn new(
        preparation: std::time::Duration,
        sequence: &Sequence,
        sound_signal: &SoundSignal,
    ) -> Self {
        Self {
            preparation: Stopwatch::from(preparation),
            sequence: sequence.clone(),
            changed: false,
            sound_signal: sound_signal.clone(),
            status: Status::default(),
        }
    }
    #[must_use]
    pub fn left(&self) -> &std::time::Duration {
        if let Some(workout) = self.sequence.current() {
            return workout.left();
        }
        self.preparation.left()
    }
    #[must_use]
    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }
    #[must_use]
    pub fn sound_signal(&self) -> &SoundSignal {
        &self.sound_signal
    }
    pub fn ring(&self) {
        self.sound_signal.ring(self.sequence.sound());
    }
    pub fn restart_sequence(&mut self) {
        self.preparation.reset();
        self.changed = true;
        self.sequence.reset();
    }
    pub fn restart_workout(&mut self) {
        self.preparation.reset();
        self.sequence.reset_workout();
    }
    pub fn tick(&mut self) -> bool {
        if self.changed {
            self.changed = false;
            return false;
        }
        if self.status.paused() {
            return false;
        }

        if self.sequence.current().is_none() && self.preparation.decrement() {
            if self.sequence.sound().is_beep() && self.preparation.last_seconds() {
                self.sound_signal.ring(self.sequence.sound());
            }
            return false;
        }
        self.preparation.reset();

        if self.sequence.decrement() {
            if self.sequence.sound().is_beep() && self.sequence.last_seconds() {
                self.sound_signal.ring(self.sequence.sound());
            }
            return false;
        }

        if !self.sequence.is_empty() && self.sequence.sound().is_bell() {
            self.sound_signal.ring(self.sequence.sound());
        }
        info!("goto next");
        if self.sequence.auto_next().is_some() {
            return true;
        }
        self.status.toggle();
        false
    }
    pub fn manual_next(&mut self) {
        if self.status().running() {
            self.changed = true;
        }
        self.sequence.manual_next();
    }
    pub fn manual_previous(&mut self) {
        if self.status().running() {
            self.changed = true;
        }
        self.sequence.goto_previous();
    }
    #[must_use]
    pub fn label(&self) -> &str {
        if self.sequence.current().is_none() {
            return defaults::PREPARE_LABEL;
        }
        let Some(workout) = self.sequence.current() else {
            return defaults::PREPARE_LABEL;
        };
        workout.item().name()
    }
    #[must_use]
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn shuffle(&mut self) {
        self.sequence.shuffle();
    }
    pub fn toggle(&mut self) {
        info!("toggle");
        self.status = self.status().next().clone();
        if self.status().running() {
            self.changed = true;
        }
    }
}
