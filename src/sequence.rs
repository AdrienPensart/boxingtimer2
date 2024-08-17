use crate::duration::DurationExt;
use crate::indexedvec::IndexedVec;
use crate::item::{Boxing, Item, Prepare, Punch, Rest, Workout};
use crate::stopwatch::Stopwatch;
use crate::tag::Tag;
use derive_more::{Deref, DerefMut, IntoIterator};
use dioxus_logger::tracing::info;
use itertools::Itertools;

#[derive(Default, PartialEq, Eq, Clone, Debug, Deref, DerefMut, Hash)]
pub struct Sequence {
    name: String,
    #[deref]
    #[deref_mut]
    items: IndexedVec<Item>,
}

impl Sequence {
    pub fn simple(name: &str, items: &[Item]) -> Self {
        Self {
            name: name.into(),
            items: IndexedVec::simple(items),
        }
    }
    pub fn cycle(name: &str, items: &[Item]) -> Self {
        Self {
            name: name.into(),
            items: IndexedVec::cycle(items),
        }
    }
    pub fn workout(
        name: &str,
        prepare: &std::time::Duration,
        duration: &std::time::Duration,
        tags: &[Tag],
    ) -> Self {
        Self {
            name: format!("{name} ({})", duration.to_string()),
            items: IndexedVec::simple(&[Prepare(prepare), Workout(duration, tags)]),
        }
    }
    pub fn stamina(
        name: &str,
        names: Vec<&str>,
        prepare: &std::time::Duration,
        workout: &std::time::Duration,
        rest: &std::time::Duration,
        times: u64,
    ) -> Self {
        let items = names.iter().map(|n| Punch(n, workout)).collect_vec();
        let rest_item = Rest(rest);
        let mut items = itertools::intersperse(vec![items; times as usize], vec![rest_item])
            .flatten()
            .collect_vec();
        if !prepare.is_zero() {
            items.insert(0, Prepare(prepare));
        }
        Self {
            name: format!(
                "{name} {}x{times}x{}s + {}s",
                names.len(),
                workout.as_secs(),
                rest.as_secs()
            ),
            items: IndexedVec::simple(&items),
        }
    }
    pub fn hiit(
        prepare: &std::time::Duration,
        workout: &std::time::Duration,
        rest: &std::time::Duration,
    ) -> Self {
        let mut items = vec![Workout(workout, &[Tag::HiiT]), Rest(rest)];
        if !prepare.is_zero() {
            items.insert(0, Prepare(prepare));
        }
        Self {
            name: format!("HiiT {}s / {}s", workout.as_secs(), rest.as_secs()),
            items: IndexedVec::cycle(&items),
        }
    }
    pub fn boxing(
        name: &str,
        rounds: usize,
        prepare: &std::time::Duration,
        workout: &std::time::Duration,
        rest: &std::time::Duration,
    ) -> Self {
        let boxing_item = Boxing(workout);
        let rest_item = Rest(rest);
        let rounds_items = vec![boxing_item; rounds];
        let mut items = itertools::intersperse(rounds_items, rest_item).collect_vec();
        if !prepare.is_zero() {
            items.insert(0, Prepare(prepare));
        }
        Self {
            name: name.into(),
            items: IndexedVec::simple(&items),
        }
    }
    // pub fn cycling(&self) -> bool {
    //     self.cycle
    // }
    // pub fn position(&self) -> &usize {
    //     &self.position
    // }
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

        if !self.items.circular() {
            self.reset();
            return false;
        }

        while let Some(item) = self.items.next_item() {
            item.reset();
            if !item.is_prepare() {
                return true;
            }
        }
        false
    }
    // pub fn last(&self) -> bool {
    //     self.position == self.items.len().saturating_sub(1)
    // }
    pub fn stopwatch(&mut self) -> Option<&Stopwatch> {
        self.items.get().map(|i| i.stopwatch())
    }
    pub fn decrement(&mut self) -> bool {
        self.items.get_mut().map_or(false, |i| i.decrement())
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
                .filter(|i| !i.is_wait())
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
            .filter(|t| **t != Tag::Prepare)
            .unique()
            .cloned()
            .collect_vec()
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
    use crate::item::{Prepare, WarmUp};
    let prepare = Prepare(&std::time::Duration::from_secs(5));
    let warm_up = WarmUp("test", &std::time::Duration::from_secs(3));
    let mut simple = Sequence::simple("simple sequence", &[prepare.clone(), warm_up.clone()]);

    assert!(simple.auto_next());
    assert_eq!(simple.get(), Some(&prepare));

    assert!(simple.auto_next());
    assert_eq!(simple.get(), Some(&warm_up));

    assert!(!simple.auto_next());
    assert_eq!(simple.get(), None);

    simple.manual_next();
    assert_eq!(simple.get(), Some(&prepare));

    simple.manual_next();
    assert_eq!(simple.get(), Some(&warm_up));

    assert!(!simple.auto_next());
    assert_eq!(simple.get(), None);

    simple.manual_next();
    assert_eq!(simple.get(), Some(&prepare));

    simple.reset();
    assert_eq!(simple.get(), None);
}

#[test]
fn sequence_cycle_tests() {
    use crate::item::{Prepare, WarmUp};
    let prepare = Prepare(&std::time::Duration::from_secs(5));
    let warm_up = WarmUp("test", &std::time::Duration::from_secs(3));
    let mut cycle = Sequence::cycle("simple sequence", &[prepare.clone(), warm_up.clone()]);

    assert_eq!(cycle.get(), None);

    assert!(cycle.auto_next());
    assert_eq!(cycle.get(), Some(&prepare));

    assert!(cycle.auto_next());
    assert_eq!(cycle.get(), Some(&warm_up));

    assert!(cycle.auto_next());
    assert_eq!(cycle.get(), Some(&warm_up));

    // cycle.manual_next();
    // assert_eq!(cycle.get(), Some(&prepare));

    // simple.goto_next(true);
    // assert_eq!(simple.get(), Some(&warm_up));

    // simple.goto_next(false);
    // assert_eq!(simple.get(), None);

    // simple.goto_next(true);
    // assert_eq!(simple.get(), Some(&prepare));

    // simple.reset();
    // assert_eq!(simple.get(), None);
}
