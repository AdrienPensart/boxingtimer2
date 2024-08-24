use crate::stopwatch::Stopwatch;
use crate::tag::Tag;
use crate::{difficulty::Difficulty, duration::DurationExt};
use derive_builder::Builder;
use derive_more::{Deref, DerefMut};
use derive_new::new;

pub enum GenericItem {
    Duration(std::time::Duration),
    Item(Item),
}

impl GenericItem {
    pub fn item(&self) -> Item {
        match self {
            Self::Item(item) => item.clone(),
            Self::Duration(duration) => Workout("Workout", *duration, &[]),
        }
    }
    pub fn builder(&self) -> ItemBuilder {
        match self {
            Self::Duration(duration) => ItemBuilder::default().stopwatch(duration).clone(),
            Self::Item(item) => {
                let name = item.name().clone().unwrap_or("Workout".into());
                let description = item.description().clone().unwrap_or_default();
                ItemBuilder::default()
                    .name(name)
                    .description(description)
                    .clone()
            }
        }
    }
}

#[derive(new, Default, Builder, Clone, Debug, Eq, PartialEq, Hash, Deref, DerefMut)]
#[builder(setter(into))]
pub struct Item {
    #[new(value = "None")]
    #[builder(setter(strip_option), default)]
    name: Option<String>,
    #[new(value = "None")]
    #[builder(default)]
    difficulty: Option<Difficulty>,
    #[deref]
    #[deref_mut]
    stopwatch: Stopwatch,
    #[new(value = "vec![]")]
    #[builder(default)]
    tags: Vec<Tag>,
    #[new(value = "None")]
    #[builder(setter(strip_option), default)]
    description: Option<String>,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if let Some(name) = &self.name {
            write!(f, "{name}")?;
            name.clone()
        } else if self.tags.len() == 1 {
            write!(f, "{}", self.tags[0])?;
            self.tags[0].to_string()
        } else {
            "".to_string()
        };
        if let Some(difficulty) = self.difficulty {
            write!(
                f,
                " - {} : {}",
                difficulty,
                self.stopwatch.duration().to_string()
            )?;
        } else {
            write!(f, " : {}", self.stopwatch.duration().to_string())?;
        }

        if !self.tags.is_empty() {
            let tags = self
                .tags
                .iter()
                .map(|&t| t.to_string())
                .collect::<Vec<_>>()
                .join(" / ");
            if name != tags {
                write!(f, " ({tags})")?;
            }
        }
        Ok(())
    }
}

impl Item {
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn stopwatch(&self) -> &Stopwatch {
        &self.stopwatch
    }
    pub fn stopwatch_mut(&mut self) -> &mut Stopwatch {
        &mut self.stopwatch
    }
    pub fn difficulty(&self) -> Option<Difficulty> {
        self.difficulty
    }
    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }
    pub fn is_prepare(&self) -> bool {
        self.tags.contains(&Tag::Prepare)
    }
    pub fn is_rest(&self) -> bool {
        self.tags.contains(&Tag::Rest)
    }
    pub fn is_wait(&self) -> bool {
        self.is_prepare() || self.is_rest()
    }
}

pub fn Easy(name: &str, duration: std::time::Duration) -> Item {
    ItemBuilder::default()
        .name(name)
        .difficulty(Difficulty::Easy)
        .stopwatch(&duration)
        .build()
        .unwrap()
}

pub fn Workout(name: &str, duration: std::time::Duration, tags: &[Tag]) -> Item {
    ItemBuilder::default()
        .name(name)
        .tags(tags)
        .stopwatch(&duration)
        .build()
        .unwrap()
}

pub fn Prepare(duration: std::time::Duration) -> Item {
    ItemBuilder::default()
        .tags([Tag::Prepare])
        .stopwatch(&duration)
        .build()
        .unwrap()
}

pub fn Rest(duration: std::time::Duration) -> Item {
    ItemBuilder::default()
        .name("Rest")
        .stopwatch(&duration)
        .build()
        .unwrap()
}
