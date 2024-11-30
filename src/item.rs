use crate::duration::DurationExt;
use crate::stopwatch::Stopwatch;
use crate::tag::Tag;
use bon::builder;
use derive_more::{Deref, DerefMut};

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash, Deref, DerefMut, bon::Builder)]
#[builder(on(String, into), on(Vec<_>, into), on(Stopwatch, into))]
pub struct Item {
    name: String,
    #[deref]
    #[deref_mut]
    stopwatch: Stopwatch,
    #[builder(default)]
    tags: Vec<Tag>,
    description: Option<String>,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} : {}",
            self.name,
            self.stopwatch.duration().to_string()
        )?;

        if !self.tags.is_empty() {
            let tags = self
                .tags
                .iter()
                .map(|&t| t.to_string())
                .collect::<Vec<_>>()
                .join(" / ");
            write!(f, " ({tags})")?;
        }
        Ok(())
    }
}

impl Item {
    pub fn name(&self) -> &String {
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
    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }
    pub fn is_rest(&self) -> bool {
        self.tags.contains(&Tag::Rest)
    }
}

pub fn Easy(name: &str, duration: std::time::Duration) -> Item {
    Item::builder()
        .name(name)
        .tags([Tag::Easy])
        .stopwatch(duration)
        .build()
}

pub fn Medium(name: &str, duration: std::time::Duration) -> Item {
    Item::builder()
        .name(name)
        .tags([Tag::Medium])
        .stopwatch(duration)
        .build()
}

pub fn Hard(name: &str, duration: std::time::Duration) -> Item {
    Item::builder()
        .name(name)
        .tags([Tag::Hard])
        .stopwatch(duration)
        .build()
}

pub fn Workout(name: &str, duration: std::time::Duration, tags: &[Tag]) -> Item {
    Item::builder()
        .name(name)
        .tags(tags)
        .stopwatch(duration)
        .build()
}

pub fn Rest(duration: std::time::Duration) -> Item {
    Item::builder()
        .name("Rest")
        .stopwatch(duration)
        .tags([Tag::Rest])
        .build()
}
