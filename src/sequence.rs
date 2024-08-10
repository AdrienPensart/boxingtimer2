use crate::duration::DurationExt;
use crate::item::{Boxing, Item, Prepare, Punch, Rest, Workout};
use crate::stopwatch::Stopwatch;
use crate::tag::Tag;
use derive_more::{Deref, DerefMut, Index, IntoIterator};
use gloo::console::log;
use itertools::Itertools;

#[derive(Default, PartialEq, Eq, Clone, Debug, IntoIterator, Index, Deref, DerefMut, Hash)]

pub struct Sequence {
    name: String,
    position: usize,
    #[deref]
    #[deref_mut]
    #[index]
    #[into_iterator]
    items: Vec<Item>,
    cycle: bool,
}

impl Sequence {
    pub fn simple(name: &str, items: &[Item]) -> Box<Self> {
        Box::new(Self {
            name: name.into(),
            items: items.into(),
            ..Default::default()
        })
    }
    pub fn cycle(name: &str, items: &[Item]) -> Box<Self> {
        Box::new(Self {
            name: name.into(),
            items: items.into(),
            cycle: true,
            ..Default::default()
        })
    }
    pub fn workout(
        name: &str,
        prepare: &std::time::Duration,
        duration: &std::time::Duration,
        tags: &[Tag],
    ) -> Box<Self> {
        Box::new(Self {
            name: format!("{name} ({})", duration.to_string()),
            items: vec![Prepare(prepare), Workout(duration, tags)],
            ..Self::default()
        })
    }
    pub fn cycling(&self) -> bool {
        self.cycle
    }
    pub fn position(&self) -> &usize {
        &self.position
    }
    pub fn goto_previous(&mut self) {
        log!("goto previous");
        if let Some(position) = self.position.checked_sub(1) {
            self.position = position
        } else {
            self.position = self.items.len() - 1
        }
        self.reset_current()
    }
    pub fn goto_next(&mut self, manual: bool) {
        log!("goto next");
        if self.position < self.items.len() - 1 {
            self.position += 1;
            self.reset_current();
        } else {
            self.reset_beginning(manual);
        }
    }
    pub fn ended(&self) -> bool {
        self.position == self.items.len().saturating_sub(1)
    }
    pub fn stopwatch(&self) -> Option<&Stopwatch> {
        self.items.get(self.position).map(|i| i.stopwatch())
    }
    pub fn decrement(&mut self) -> bool {
        self.items
            .get_mut(self.position)
            .map_or(false, |i| i.decrement())
    }
    pub fn reset_current(&mut self) {
        if let Some(i) = self.items.get_mut(self.position) {
            i.reset()
        }
    }
    pub fn reset(&mut self) {
        self.position = 0;
        self.items.iter_mut().for_each(|item| item.reset());
    }
    pub fn reset_beginning(&mut self, manual: bool) {
        self.position = 0;
        if !manual && self.cycle {
            self.position = 1;
        }
        self.reset_current()
    }
    pub fn stamina(
        name: &str,
        names: Vec<&str>,
        prepare: &std::time::Duration,
        workout: &std::time::Duration,
        rest: &std::time::Duration,
        times: u64,
    ) -> Box<Self> {
        let items = names.iter().map(|n| Punch(n, workout)).collect_vec();
        let rest_item = Rest(rest);
        let mut items = itertools::intersperse(vec![items; times as usize], vec![rest_item])
            .flatten()
            .collect_vec();
        if !prepare.is_zero() {
            items.insert(0, Prepare(prepare));
        }
        Box::new(Self {
            name: format!(
                "{name} {}x{times}x{}s + {}s",
                names.len(),
                workout.as_secs(),
                rest.as_secs()
            ),
            items,
            ..Self::default()
        })
    }
    pub fn hiit(
        prepare: &std::time::Duration,
        workout: &std::time::Duration,
        rest: &std::time::Duration,
    ) -> Box<Self> {
        let mut items = vec![Rest(rest), Workout(workout, &[Tag::HiiT])];
        if !prepare.is_zero() {
            items.insert(0, Prepare(prepare));
        }
        Box::new(Self {
            name: format!("HiiT {}s / {}s", workout.as_secs(), rest.as_secs()),
            items,
            cycle: true,
            ..Self::default()
        })
    }
    pub fn boxing(
        name: &str,
        rounds: usize,
        prepare: &std::time::Duration,
        workout: &std::time::Duration,
        rest: &std::time::Duration,
    ) -> Box<Self> {
        let boxing_item = Boxing(workout);
        let rest_item = Rest(rest);
        let rounds_items = vec![boxing_item; rounds];
        let mut items = itertools::intersperse(rounds_items, rest_item).collect_vec();
        if !prepare.is_zero() {
            items.insert(0, Prepare(prepare));
        }
        Box::new(Self {
            name: name.into(),
            items,
            ..Self::default()
        })
    }
    pub fn name(&self) -> String {
        let cycle = if self.cycle { " ⟳" } else { "" };
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
                .filter(|i| !i.waiting())
                .map(|i| i.stopwatch().duration().as_secs())
                .sum(),
        )
    }
    pub fn waiting_total(&self) -> std::time::Duration {
        std::time::Duration::from_secs(
            self.iter()
                .filter(|i| i.waiting())
                .map(|i| i.stopwatch().duration().as_secs())
                .sum(),
        )
    }
    pub fn tags_str(&self) -> String {
        self.items
            .iter()
            .flat_map(|i| i.tags())
            .filter(|t| **t != Tag::Prepare)
            .unique()
            .join(", ")
    }
}

impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tags = self.tags_str();
        let name = self.name();
        if name.contains(&tags) {
            write!(f, "{name}")
        } else {
            write!(f, "{name} ({tags})")
        }
    }
}
