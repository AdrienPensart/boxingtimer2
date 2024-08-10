use crate::duration::Duration;
use crate::item::{Boxing, Item, Prepare, Punch, Rest, Workout};
use crate::stopwatch::Stopwatch;
use crate::tag::Tag;
use derive_more::{Deref, DerefMut, Index, IntoIterator};
use derive_new::new;
use gloo::console::log;
use itertools::Itertools;

#[derive(new, Default, PartialEq, Eq, Clone, Debug, IntoIterator, Index, Deref, DerefMut, Hash)]

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

impl Sequence {
    pub fn workout(name: &str, prepare: &Duration, duration: &Duration, tags: &[Tag]) -> Self {
        Self {
            name: format!("{name} ({duration})"),
            items: vec![Prepare(prepare), Workout(duration, tags)],
            ..Self::default()
        }
    }
    pub fn cycle(&self) -> bool {
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
        self.position == self.items.len()
    }
    pub fn stopwatch(&self) -> &Stopwatch {
        self.items[self.position].stopwatch()
    }
    pub fn decrement(&mut self) -> bool {
        self.items[self.position].stopwatch_mut().decrement()
    }
    pub fn reset_current(&mut self) {
        self.items[self.position].stopwatch_mut().reset()
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
        prepare: &Duration,
        workout: &Duration,
        rest: &Duration,
        times: u64,
    ) -> Self {
        let items = names.iter().map(|n| Punch(n, workout)).collect_vec();
        let rest_item = Rest(rest);
        let mut items = itertools::intersperse(vec![items; times as usize], vec![rest_item])
            .flatten()
            .collect_vec();
        items.insert(0, Prepare(prepare));
        Self {
            name: format!(
                "{name} {}x{times}x{}s + {}s",
                names.len(),
                workout.as_secs(),
                rest.as_secs()
            ),
            items,
            ..Self::default()
        }
    }
    pub fn hiit(prepare: &Duration, workout: &Duration, rest: &Duration) -> Self {
        Self {
            name: format!("HiiT {}s / {}s", workout.as_secs(), rest.as_secs()),
            items: vec![Prepare(prepare), Rest(rest), Workout(workout, &[Tag::HiiT])],
            cycle: true,
            ..Self::default()
        }
    }
    pub fn boxing(
        name: &str,
        rounds: usize,
        prepare: &Duration,
        workout: &Duration,
        rest: &Duration,
    ) -> Self {
        let boxing_item = Boxing(workout);
        let rest_item = Rest(rest);
        let rounds_items = vec![boxing_item; rounds];
        let mut items = itertools::intersperse(rounds_items, rest_item).collect_vec();
        items.insert(0, Prepare(prepare));
        Self {
            name: name.into(),
            items,
            ..Self::default()
        }
    }
    pub fn name(&self) -> String {
        let cycle = if self.cycle { " ⟳" } else { "" };
        format!("{}{}", self.name, cycle)
    }
    pub fn total(&self) -> Duration {
        Duration::from_secs(
            self.iter()
                .map(|item| item.stopwatch().duration().as_secs())
                .sum(),
        )
    }
    pub fn left_total(&self) -> Duration {
        Duration::from_secs(
            self.iter()
                .map(|item| item.stopwatch().left().as_secs())
                .sum(),
        )
    }
    pub fn workout_total(&self) -> Duration {
        Duration::from_secs(
            self.iter()
                .filter(|i| !i.waiting())
                .map(|i| i.stopwatch().duration().as_secs())
                .sum(),
        )
    }
    pub fn waiting_total(&self) -> Duration {
        Duration::from_secs(
            self.iter()
                .filter(|i| i.waiting())
                .map(|i| i.stopwatch().duration().as_secs())
                .sum(),
        )
    }
    pub fn tags_str(&self) -> String {
        self.items.iter().flat_map(|i| i.tags()).unique().join(", ")
    }
}
