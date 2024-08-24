use crate::duration::DurationExt;
use crate::indexedvec::IndexedVec;
use crate::sequence::Sequence;
use crate::status::Status;
use dioxus_logger::tracing::info;
pub const DEFAULT_INTERVAL: u32 = 1000;

#[derive(Default, Clone)]
pub struct Timer {
    status: Status,
    sequences: IndexedVec<Sequence>,
    elapsed: std::time::Duration,
    changed: bool,
}

impl Timer {
    pub fn new(sequences: &[Sequence]) -> Self {
        Self {
            sequences: IndexedVec::simple(sequences),
            ..Default::default()
        }
    }
    pub fn sequences(&self) -> &IndexedVec<Sequence> {
        &self.sequences
    }
    pub fn always_ring(&self) -> bool {
        if let Some(sequence) = self.sequences.get() {
            sequence.signal().always_ring();
            true
        } else {
            false
        }
    }
    pub fn set_sequence(&mut self, index: usize) {
        info!("timer: setting sequence of index {index}");
        if let Some(s) = self.sequences.set_index(index) {
            s.reset();
            if self.status.running() {
                s.set_index(0);
            }
        } else {
            info!("timer: no current sequence")
        }
    }
    pub fn restart_sequence(&mut self) {
        if let Some(sequence) = self.sequences.get_mut() {
            self.changed = true;
            sequence.reset();
        }
    }
    pub fn restart_item(&mut self) {
        if let Some(sequence) = self.sequences.get_mut() {
            self.changed = true;
            sequence.reset_current()
        }
    }
    pub fn tick(&mut self) -> bool {
        if self.changed {
            self.changed = false;
            return false;
        }
        if self.status.paused() {
            // log!("paused, skipping tick");
            return false;
        }

        self.elapsed.increment();
        let Some(sequence) = self.sequences.get_mut() else {
            return false;
        };

        if sequence.signal().sound().is_beep() && sequence.last_seconds() {
            sequence.signal().ring();
        }
        if !sequence.decrement() {
            if !sequence.is_empty() && sequence.signal().sound().is_bell() {
                sequence.signal().ring();
            }
            info!("goto next");
            if sequence.auto_next() {
                return true;
            }
            self.status.toggle();
        }
        false
    }
    pub fn manual_next(&mut self) {
        if self.running() {
            self.changed = true;
        }
        if let Some(sequence) = self.sequences.get_mut() {
            sequence.manual_next();
        }
    }
    pub fn manual_previous(&mut self) {
        if self.running() {
            self.changed = true;
        }
        if let Some(sequence) = self.sequences.get_mut() {
            sequence.goto_previous();
        }
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn toggle(&mut self) {
        if self.sequences.get().is_none() {
            return;
        }
        if self.sequences.is_empty() {
            return;
        }
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
    pub fn elapsed(&self) -> &std::time::Duration {
        &self.elapsed
    }
}

// #[test]
// fn timer_tests() {
//     use crate::item::{Easy, Prepare};
//     use crate::signal::SILENT;
//     let prepare = Prepare(&std::time::Duration::from_secs(5));
//     let warm_up = Easy("test", &std::time::Duration::from_secs(3));
//     let first_sequence = Sequence::simple(
//         "first sequence",
//         &[prepare.clone(), warm_up.clone()],
//         &[],
//         &SILENT,
//     );
//     let second_sequence = Sequence::simple(
//         "double sequence",
//         &[prepare.clone(), warm_up.clone()],
//         &[],
//         &SILENT,
//     );
//     let mut timer = Timer::new(&[first_sequence.clone(), second_sequence.clone()]);

//     assert_eq!(timer.sequences.get(), None);

//     timer.set_sequence(0);
//     assert_eq!(timer.sequences.get(), Some(&first_sequence));
//     if let Some(sequence) = timer.sequences.get_mut() {
//         sequence.manual_next();
//     }
//     assert_ne!(timer.sequences.get(), Some(&first_sequence));

//     timer.set_sequence(1);
//     assert_eq!(timer.sequences.get(), Some(&second_sequence));
//     if let Some(sequence) = timer.sequences.get_mut() {
//         sequence.manual_next();
//     }
//     assert_ne!(timer.sequences.get(), Some(&second_sequence));

//     timer.set_sequence(2);
//     assert_eq!(timer.sequences.get(), None);
// }
