use crate::item::Item;
use crate::item_list::ItemList;
use crate::tag::Tag;
use crate::workout::Workout;
use itertools::Itertools;
use test_log::test;

pub enum Exercises {
    Items(Vec<Item>),
    Names(Vec<String>, Option<char>),
}

impl Exercises {
    pub fn from_tags(tags: &[Tag]) -> Self {
        Self::Items(
            ItemList::tag_to_items()
                .iter()
                .filter(|(tag, _)| tags.contains(tag))
                .flat_map(|(_, items)| items.iter().cloned())
                .unique()
                .collect_vec(),
        )
    }
    pub fn from_strings(names: Vec<&str>, icon: Option<char>) -> Self {
        Self::Names(names.into_iter().map(str::to_string).collect_vec(), icon)
    }
    pub fn from_items(items: Vec<Item>) -> Self {
        Self::Items(items)
    }
    pub fn len(&self) -> usize {
        match self {
            Self::Items(items) => items.len(),
            Self::Names(names, _) => names.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
                        .icon(icon.unwrap_or(crate::defaults::DEFAULT_ICON))
                        .build()
                        .workout(duration)
                })
                .collect_vec(),
        }
    }
}

impl std::fmt::Display for Exercises {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Items(items) => write!(f, "{}", items.iter().map(Item::name).join(", ")),
            Self::Names(names, _) => write!(f, "{}", names.join(", ")),
        }
    }
}

#[test]
fn exercises_tests() {
    use crate::tag::Body;
    use strum::VariantArray;
    let body_tags = Body::VARIANTS.iter().map(|v| Tag::Body(*v)).collect_vec();
    let exercises = Exercises::from_tags(&body_tags);
    assert_eq!(
        exercises.len(),
        3,
        "exercises for tags {:?} : {}",
        body_tags,
        exercises
    );
}
