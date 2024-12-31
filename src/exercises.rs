use crate::defaults;
use crate::item::Item;
use crate::workout::Workout;
use itertools::Itertools;

pub enum Exercises {
    Items(Vec<Item>),
    Names(Vec<String>, Option<char>),
}

impl Exercises {
    pub fn from(names: Vec<&str>, icon: Option<char>) -> Self {
        Self::Names(names.into_iter().map(str::to_string).collect_vec(), icon)
    }
    pub fn workouts(self, duration: std::time::Duration) -> Vec<Workout> {
        match self {
            Self::Items(items) => items
                .into_iter()
                .map(|item| item.workout(duration))
                .collect_vec(),
            Self::Names(names, icon) => names
                .into_iter()
                .map(|name| {
                    Item::builder()
                        .name(name)
                        .icon(icon.unwrap_or(defaults::DEFAULT_ICON))
                        .build()
                        .workout(duration)
                })
                .collect_vec(),
        }
    }
}
