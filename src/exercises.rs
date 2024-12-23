use itertools::Itertools;

use crate::item::Item;
use crate::workout::Workout;

pub enum Exercises {
    Items(Vec<Item>),
    Names(Vec<String>),
}

impl Exercises {
    pub fn from(names: Vec<&str>) -> Self {
        Self::Names(names.iter().map(|name| name.to_string()).collect_vec())
    }
    pub fn workouts(&self, duration: std::time::Duration) -> Vec<Workout> {
        match self {
            Self::Items(items) => items
                .iter()
                .map(|item| item.workout(duration))
                .collect_vec(),
            Self::Names(names) => names
                .iter()
                .map(|name| Item::builder().name(name).build().workout(duration))
                .collect_vec(),
        }
    }
}
