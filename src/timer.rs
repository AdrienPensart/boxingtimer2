// use crate::audio;
// use crate::signal::SoundSignal;
// use crate::status::Status;
// use dioxus::logger::tracing::info;
// use sport::defaults;
// use sport::duration::DurationExt;
// use sport::indexedvec::IndexedVec;
// use sport::sequence::Sequence;
// use sport::stopwatch::Stopwatch;
// use sport::workout::Workout;

// #[derive(Clone)]
// pub struct Timer {
//     status: Status,
//     sequences: IndexedVec<Sequence>,
//     preparation: Stopwatch,
//     elapsed: std::time::Duration,
//     changed: bool,
//     sound_signal: SoundSignal,
// }

// impl Timer {
//     #[must_use]
//     pub fn from_sequence(sequence: Sequence, sound_signal: &SoundSignal) -> Self {
//         Self::new(defaults::PREPARE_DURATION, &[sequence], sound_signal)
//     }
//     #[must_use]
//     pub fn new(
//         preparation: std::time::Duration,
//         sequences: &[Sequence],
//         sound_signal: &SoundSignal,
//     ) -> Self {
//         Self {
//             preparation: Stopwatch::from(preparation),
//             sequences: sequences.into(),
//             changed: false,
//             sound_signal: sound_signal.clone(),
//             status: Status::default(),
//             elapsed: std::time::Duration::default(),
//         }
//     }
//     #[must_use]
//     pub fn left(&self) -> &std::time::Duration {
//         if let Some(sequence) = self.sequences.current() {
//             if let Some(workout) = sequence.current() {
//                 return workout.left();
//             }
//         }
//         self.preparation.left()
//     }
//     #[must_use]
//     pub fn sequences(&self) -> &IndexedVec<Sequence> {
//         &self.sequences
//     }
//     pub fn always_ring(&self) {
//         if let Some(sequence) = self.sequences.current() {
//             let _ = audio::play(sequence.sound());
//         }
//     }
//     pub fn set_sequence_by_slug(&mut self, slug: &str) {
//         if slug.is_empty() {
//             return;
//         }
//         for index in 0..self.sequences.len() {
//             if self.sequences[index].slug() == slug {
//                 self.set_sequence(index);
//             }
//         }
//     }
//     pub fn set_sequence(&mut self, index: usize) -> Option<String> {
//         self.preparation.reset();
//         info!("timer: setting sequence of index {index}");
//         if let Some(s) = self.sequences.set_index(index) {
//             s.reset();
//             Some(s.slug())
//         } else {
//             info!("timer: no current sequence");
//             None
//         }
//     }
//     pub fn restart_sequence(&mut self) {
//         self.preparation.reset();
//         if let Some(sequence) = self.sequences.current_mut() {
//             self.changed = true;
//             sequence.reset();
//         }
//     }
//     pub fn get_next_workout(&mut self) -> Option<&Workout> {
//         if let Some(sequence) = self.sequences.current_mut() {
//             return sequence.next_workout();
//         }
//         None
//     }
//     pub fn restart_workout(&mut self) {
//         if let Some(sequence) = self.sequences.current_mut() {
//             if sequence.current().is_none() {
//                 self.preparation.reset();
//             } else {
//                 self.changed = true;
//                 sequence.reset_workout();
//             }
//         }
//     }
//     pub fn tick(&mut self) -> bool {
//         if self.changed {
//             self.changed = false;
//             return false;
//         }
//         if self.status.paused() {
//             return false;
//         }

//         self.elapsed.increment();
//         let Some(sequence) = self.sequences.current_mut() else {
//             return false;
//         };

//         if sequence.current().is_none() && self.preparation.decrement() {
//             if sequence.sound().is_beep() && self.preparation.last_seconds() {
//                 self.sound_signal.ring(sequence.sound());
//             }
//             return false;
//         }
//         self.preparation.reset();

//         if sequence.decrement() {
//             if sequence.sound().is_beep() && sequence.last_seconds() {
//                 self.sound_signal.ring(sequence.sound());
//             }
//             return false;
//         }

//         if !sequence.is_empty() && sequence.sound().is_bell() {
//             self.sound_signal.ring(sequence.sound());
//         }
//         info!("goto next");
//         if sequence.auto_next().is_some() {
//             return true;
//         }
//         self.status.toggle();
//         false
//     }
//     pub fn manual_next(&mut self) {
//         if self.running() {
//             self.changed = true;
//         }
//         if let Some(sequence) = self.sequences.current_mut() {
//             sequence.manual_next();
//         }
//     }
//     pub fn manual_previous(&mut self) {
//         if self.running() {
//             self.changed = true;
//         }
//         if let Some(sequence) = self.sequences.current_mut() {
//             sequence.goto_previous();
//         }
//     }
//     #[must_use]
//     pub fn label(&self) -> &str {
//         let Some(sequence) = self.sequences.current() else {
//             return defaults::PREPARE_LABEL;
//         };
//         let Some(workout) = sequence.current() else {
//             return defaults::PREPARE_LABEL;
//         };
//         workout.item().name()
//     }
//     #[must_use]
//     pub fn status(&self) -> &Status {
//         &self.status
//     }
//     pub fn shuffle(&mut self) {
//         if let Some(sequence) = self.sequences.current_mut() {
//             sequence.shuffle();
//         }
//     }
//     pub fn toggle(&mut self) {
//         if self.sequences.current().is_none() {
//             return;
//         }
//         if self.sequences.is_empty() {
//             return;
//         }
//         self.status = self.next_status().clone();
//         if self.running() {
//             self.changed = true;
//         }
//     }
//     #[must_use]
//     pub fn next_status(&self) -> &Status {
//         self.status.next()
//     }
//     #[must_use]
//     pub fn paused(&self) -> bool {
//         self.status.paused()
//     }
//     #[must_use]
//     pub fn running(&self) -> bool {
//         self.status.running()
//     }
//     #[must_use]
//     pub fn elapsed(&self) -> &std::time::Duration {
//         &self.elapsed
//     }
// }

// #[test]
// fn timer_tests() {
//     use sport::item::Item;
//     use sport::sound::Sound;
//     use time::ext::NumericalStdDuration;
//     let preparation = std::time::Duration::from_secs(5);
//     let warm_up = Item::builder()
//         .name("test")
//         .build()
//         .workout(3.std_seconds());
//     let first_sequence = Sequence::simple()
//         .name("first sequence")
//         .workouts(&[warm_up.clone()])
//         .sound(&Sound::Silent)
//         .call();

//     let second_sequence = Sequence::simple()
//         .name("double sequence")
//         .workouts(&[warm_up.clone()])
//         .sound(&Sound::Silent)
//         .call();
//     let mut timer = Timer::new(
//         preparation,
//         &[first_sequence.clone(), second_sequence.clone()],
//         &SoundSignal::default(),
//     );

//     assert_eq!(timer.sequences.current(), None);

//     timer.set_sequence(0);
//     assert_eq!(timer.sequences.current(), Some(&first_sequence));
//     if let Some(sequence) = timer.sequences.current_mut() {
//         sequence.manual_next();
//     }
//     assert_ne!(timer.sequences.current(), Some(&first_sequence));

//     timer.set_sequence(1);
//     assert_eq!(timer.sequences.current(), Some(&second_sequence));
//     if let Some(sequence) = timer.sequences.current_mut() {
//         sequence.manual_next();
//     }
//     assert_ne!(timer.sequences.current(), Some(&second_sequence));

//     timer.set_sequence(2);
//     assert_eq!(timer.sequences.current(), None);
// }
