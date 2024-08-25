use crate::duration::DurationExt;
use crate::stopwatch::Stopwatch;
use crate::tag::Tag;
use derive_builder::Builder;
use derive_more::{Deref, DerefMut};
use derive_new::new;

#[derive(new, Default, Builder, Clone, Debug, Eq, PartialEq, Hash, Deref, DerefMut)]
#[builder(setter(into))]
pub struct Item {
    name: String,
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
    ItemBuilder::default()
        .name(name)
        .tags([Tag::Easy])
        .stopwatch(duration)
        .build()
        .unwrap()
}

pub fn Medium(name: &str, duration: std::time::Duration) -> Item {
    ItemBuilder::default()
        .name(name)
        .tags([Tag::Medium])
        .stopwatch(duration)
        .build()
        .unwrap()
}

pub fn Hard(name: &str, duration: std::time::Duration) -> Item {
    ItemBuilder::default()
        .name(name)
        .tags([Tag::Hard])
        .stopwatch(duration)
        .build()
        .unwrap()
}

pub fn Workout(name: &str, duration: std::time::Duration, tags: &[Tag]) -> Item {
    ItemBuilder::default()
        .name(name)
        .tags(tags)
        .stopwatch(duration)
        .build()
        .unwrap()
}

pub fn Rest(duration: std::time::Duration) -> Item {
    ItemBuilder::default()
        .name("Rest")
        .stopwatch(duration)
        .tags([Tag::Rest])
        .build()
        .unwrap()
}
