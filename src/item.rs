use crate::difficulty::Difficulty;
use crate::duration::Duration;
use crate::stopwatch::Stopwatch;
use crate::tag::Tag;
use derive_builder::Builder;
use derive_more::{Deref, DerefMut};
use derive_new::new;

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
    #[new(value = "false")]
    #[builder(default)]
    waiting: bool,
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
            write!(f, " - {} : {}", difficulty, self.stopwatch.duration())?;
        } else {
            write!(f, " : {}", self.stopwatch.duration())?;
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
    pub fn waiting(&self) -> bool {
        self.waiting
    }
}

pub fn Boxing(duration: &Duration) -> Item {
    ItemBuilder::default()
        .difficulty(Difficulty::Medium)
        .stopwatch(duration)
        .tags([Tag::Boxing])
        .build()
        .unwrap()
}

pub fn Punch(name: &str, duration: &Duration) -> Item {
    ItemBuilder::default()
        .name(name)
        .tags([Tag::Boxing])
        .stopwatch(duration)
        .difficulty(Difficulty::Medium)
        .build()
        .unwrap()
}

pub fn Workout(duration: &Duration, tags: &[Tag]) -> Item {
    ItemBuilder::default()
        .name("Workout")
        .tags(tags)
        .stopwatch(duration)
        .build()
        .unwrap()
}

pub fn Prepare(duration: &Duration) -> Item {
    ItemBuilder::default()
        .waiting(true)
        .tags([Tag::Prepare])
        .stopwatch(duration)
        .build()
        .unwrap()
}

pub fn Maintain(duration: &Duration) -> Item {
    ItemBuilder::default()
        .name("Maintain")
        .difficulty(Difficulty::Hard)
        .waiting(true)
        .stopwatch(duration)
        .build()
        .unwrap()
}

pub fn Contract(duration: &Duration) -> Item {
    ItemBuilder::default()
        .name("Contract")
        .difficulty(Difficulty::Hard)
        .waiting(true)
        .stopwatch(duration)
        .build()
        .unwrap()
}

pub fn Rest(duration: &Duration) -> Item {
    ItemBuilder::default()
        .name("Rest")
        .stopwatch(duration)
        .waiting(true)
        .build()
        .unwrap()
}
