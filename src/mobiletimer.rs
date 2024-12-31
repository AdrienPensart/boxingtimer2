use crate::defaults;
use crate::sequence::Sequence;
use crate::signal::SoundSignal;
use crate::status::Status;
use crate::stopwatch::Stopwatch;
use crate::workout::Workout;
use dioxus::logger::tracing::info;

#[derive(Debug)]
pub struct MobileTimer {
    status: Status,
    sequence: Sequence,
    preparation: Stopwatch,
    changed: bool,
    sound_signal: SoundSignal,
}

impl MobileTimer {
    pub fn from_sequence(sequence: &Sequence, sound_signal: &SoundSignal) -> Self {
        Self::new(defaults::PREPARE_DURATION, sequence, sound_signal)
    }
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
    pub fn left(&self) -> &std::time::Duration {
        if let Some(workout) = self.sequence.get() {
            return workout.left();
        }
        self.preparation.left()
    }
    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }
    pub fn always_ring(&self) {
        let _ = self.sequence.sound().play();
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

        if self.sequence.get().is_none() && self.preparation.decrement() {
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
        let Some(workout) = self.sequence.get() else {
            return defaults::PREPARE_LABEL;
        };
        workout.item().name()
    }
    pub fn current_workout(&self) -> Option<&Workout> {
        self.sequence.get()
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
