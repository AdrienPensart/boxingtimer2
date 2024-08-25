use crate::indexedvec::IndexedVec;
use crate::item::{Item, Rest, Workout};
use crate::signal::Signal;
use crate::stopwatch::Stopwatch;
use crate::tag::Tag;
use derive_more::{Deref, DerefMut, IntoIterator};
use dioxus_logger::tracing::info;
use itertools::Itertools;
use rand::seq::SliceRandom;

#[derive(Debug, Default, PartialEq, Eq, Clone, Deref, DerefMut)]
pub struct Sequence {
    name: String,
    #[deref]
    #[deref_mut]
    items: IndexedVec<Item>,
    signal: Signal,
    rest: Option<std::time::Duration>,
}

pub enum SmartSequence {
    // simple
    Simple(Sequence),
    // random
    Random(Sequence),
    // infinite
    Infinite(Sequence),
    // repeat
    Repeat(Sequence, usize),
}

type Rounds = u64;
pub static ROUNDS: Rounds = 1;

impl Sequence {
    pub fn random(name: &str, items: &[Item], rest: std::time::Duration, signal: &Signal) -> Self {
        let mut rng = rand::thread_rng();
        let mut items = items.to_vec();
        items.shuffle(&mut rng);
        items = itertools::intersperse(items.to_vec(), Rest(rest)).collect_vec();
        Self {
            name: name.into(),
            items: IndexedVec::simple(&items),
            signal: signal.clone(),
            rest: Some(rest),
        }
    }
    pub fn simple(name: &str, items: &[Item], signal: &Signal) -> Self {
        Self {
            name: name.into(),
            items: IndexedVec::simple(items),
            signal: signal.clone(),
            rest: None,
        }
    }
    pub fn infinite(workout: Item, rest: std::time::Duration, signal: &Signal) -> Self {
        let seconds = workout.stopwatch().duration().as_secs();
        let items = vec![workout, Rest(rest)];
        Self {
            name: format!("HiiT {seconds}s / {}s", rest.as_secs()),
            items: IndexedVec::cycle(&items),
            signal: signal.clone(),
            rest: None,
        }
    }
    pub fn workout(name: &str, workout: Item, signal: &Signal) -> Self {
        Self {
            name: name.into(),
            items: IndexedVec::simple(&[workout]),
            signal: signal.clone(),
            rest: None,
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn repeat(
        name: &str,
        names: Vec<&str>,
        workout: std::time::Duration,
        rounds: Rounds,
        rest: std::time::Duration,
        tags: &[Tag],
        signal: &Signal,
    ) -> Self {
        let items = names
            .iter()
            .map(|n| Workout(n, workout, tags))
            .collect_vec();
        let items = itertools::intersperse(vec![items; rounds as usize], vec![Rest(rest)])
            .flatten()
            .collect_vec();
        Self {
            name: name.into(),
            items: IndexedVec::simple(&items),
            signal: signal.clone(),
            rest: None,
        }
    }
    pub fn rounds(
        name: &str,
        rounds: Rounds,
        workout: Item,
        rest: std::time::Duration,
        signal: &Signal,
    ) -> Self {
        let rounds_items = vec![workout; rounds as usize];
        let items = itertools::intersperse(rounds_items, Rest(rest)).collect_vec();
        Self {
            name: name.into(),
            items: IndexedVec::simple(&items),
            signal: signal.clone(),
            rest: None,
        }
    }
    pub fn goto_previous(&mut self) -> Option<&mut Item> {
        info!("sequence: goto previous");
        if self.items.is_empty() {
            info!("sequence: items is empty, no previous");
            return None;
        }

        // if let Some(position) = self.position.checked_sub(1) {
        //     self.position = position
        // } else {
        //     self.position = self.items.len() - 1
        // }
        self.items.previous_item().map(|p| {
            p.reset();
            p
        })
    }
    pub fn manual_next(&mut self) -> Option<&mut Item> {
        info!("sequence: manual next");
        if self.items.is_empty() {
            info!("sequence: items is empty, no next");
        } else if !self.items.last() {
            self.items.next_item();
            self.reset_current();
        } else {
            self.items.set_index(0);
            self.reset_current();
        }
        self.get_mut()
    }
    pub fn auto_next(&mut self) -> bool {
        info!("sequence: auto next");
        if self.items.is_empty() {
            info!("sequence: items is empty, no next");
            return false;
        }
        if !self.items.last() {
            self.items.next_item();
            self.reset_current();
            return true;
        }

        if self.items.circular() {
            self.reset();
            return true;
        }

        // if let Some(item) = self.items.next_item() {
        //     item.reset();
        // }

        false
    }
    pub fn stopwatch(&mut self) -> Option<&Stopwatch> {
        self.items.get().map(|i| i.stopwatch())
    }
    pub fn decrement(&mut self) -> bool {
        self.items.get_mut().map_or(false, |i| i.decrement())
    }
    pub fn last_seconds(&self) -> bool {
        self.items.get().map_or(false, |i| i.last_seconds())
    }
    pub fn reset_current(&mut self) {
        info!("sequence: reset current item");
        if let Some(i) = self.items.get_mut() {
            i.reset()
        }
    }
    pub fn reset(&mut self) {
        info!("sequence: reset all items");
        self.items.reset();
        self.items.apply(|item| item.reset());
    }
    pub fn name(&self) -> String {
        let cycle = if self.circular() { " ⟳" } else { "" };
        format!("{}{}", self.name, cycle)
    }
    pub fn total(&self) -> std::time::Duration {
        std::time::Duration::from_secs(
            self.iter()
                .map(|item| item.stopwatch().duration().as_secs())
                .sum(),
        )
    }
    pub fn left_total(&self) -> std::time::Duration {
        std::time::Duration::from_secs(
            self.iter()
                .map(|item| item.stopwatch().left().as_secs())
                .sum(),
        )
    }
    pub fn workout_total(&self) -> std::time::Duration {
        std::time::Duration::from_secs(
            self.iter()
                .filter(|i| !i.is_rest())
                .map(|i| i.stopwatch().duration().as_secs())
                .sum(),
        )
    }
    pub fn rest_total(&self) -> std::time::Duration {
        std::time::Duration::from_secs(
            self.iter()
                .filter(|i| i.is_rest())
                .map(|i| i.stopwatch().duration().as_secs())
                .sum(),
        )
    }
    pub fn tags(&self) -> Vec<Tag> {
        self.items
            .iter()
            .flat_map(|i| i.tags())
            .filter(|t| **t != Tag::Rest)
            .unique()
            .cloned()
            .collect_vec()
    }
    pub fn signal(&self) -> &Signal {
        &self.signal
    }
    pub fn shuffle(&mut self) {
        // it was interspersed with rest, rebuild sequence
        if let Some(rest) = self.rest {
            let rest = Rest(rest);
            self.items.retain(&rest);
            let mut items = self.items.shuffled();
            items = itertools::intersperse(items, rest).collect_vec();
            self.items = IndexedVec::simple(&items);
        } else {
            self.items.shuffle()
        }
    }
}

impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tags = self.tags().into_iter().join(",");
        let name = self.name();
        if name.contains(&tags) {
            write!(f, "{name}")
        } else {
            write!(f, "{name} ({tags})")
        }
    }
}

#[test]
fn sequence_simple_tests() {
    use crate::duration::SECOND;
    use crate::item::{Easy, Medium};
    use crate::signal::Signal;
    let none = Signal::none();
    let warm_up = Easy("test", 3 * SECOND);
    let workout = Medium("workout", 6 * SECOND);
    let mut simple = Sequence::simple(
        "simple sequence",
        &[warm_up.clone(), workout.clone()],
        &none,
    );

    assert!(simple.auto_next());
    assert_eq!(simple.get(), Some(&warm_up));

    assert!(simple.auto_next());
    assert_eq!(simple.get(), Some(&workout));

    assert!(!simple.auto_next());
    assert_eq!(simple.get(), None);

    simple.manual_next();
    assert_eq!(simple.get(), Some(&warm_up));

    simple.manual_next();
    assert_eq!(simple.get(), Some(&workout));

    assert!(!simple.auto_next());
    assert_eq!(simple.get(), None);

    simple.manual_next();
    assert_eq!(simple.get(), Some(&warm_up));

    simple.reset();
    assert_eq!(simple.get(), None);
}

// #[test]
// fn sequence_cycle_tests() {
//     use crate::duration::SECOND;
//     use crate::item::Medium;
//     use crate::signal::Signal;
//     let none = Signal::none();

//     let workout = Medium("workout", 6 * SECOND);
//     let _30_seconds = 30 * SECOND;
//     let rest = Rest(_30_seconds);

//     let mut cycle = Sequence::infinite(workout.clone(), _30_seconds, &none);

//     assert_eq!(cycle.get(), None);

//     assert!(cycle.auto_next());
//     assert_eq!(cycle.get(), Some(&workout));

//     assert!(cycle.auto_next());
//     assert_eq!(cycle.get(), Some(&rest));

//     assert!(cycle.auto_next());
//     assert_eq!(cycle.get(), Some(&workout));

//     cycle.manual_next();
//     assert_eq!(cycle.get(), Some(&rest));
// }
