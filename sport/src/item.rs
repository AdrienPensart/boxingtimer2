use crate::tag::{Difficulty, Tag, Tags};
use crate::workout::Workout;
use bon::Builder;
use derive_more::Display;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use slug::slugify;

#[derive(Default, Display, Clone, Debug, Eq, PartialEq, Hash, Builder, Serialize, Deserialize)]
#[display("{}{name}", icon.unwrap_or('‎'))]
pub struct Item {
    #[builder(into)]
    name: String,
    #[builder(default, into)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Tags,
    #[builder(into)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[builder(into)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    difficulty: Option<Difficulty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    icon: Option<char>,
}

impl Item {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn icon(&self) -> Option<char> {
        self.icon
    }
    pub fn slug(&self) -> String {
        slugify(&self.name)
    }
    pub fn tags(&self) -> &Tags {
        &self.tags
    }
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn is_rest(&self) -> bool {
        self.tags.contains(&Tag::Rest)
    }
    pub fn joined_tags(&self) -> Option<String> {
        if !self.tags.is_empty() {
            let joined = self.tags.iter().map(Tag::to_string).join(" / ");
            if !joined.is_empty() {
                return Some(joined);
            }
        }
        None
    }
    pub fn workout(&self, duration: std::time::Duration) -> Workout {
        Workout::builder()
            .item(self.clone())
            .stopwatch(duration)
            .build()
    }
    pub fn difficulty(&self, duration: std::time::Duration, difficulty: Difficulty) -> Workout {
        Workout::builder()
            .item(self.clone())
            .stopwatch(duration)
            .difficulty(difficulty)
            .build()
    }
    pub fn easy(&self, duration: std::time::Duration) -> Workout {
        self.difficulty(duration, Difficulty::Easy)
    }
    pub fn medium(&self, duration: std::time::Duration) -> Workout {
        self.difficulty(duration, Difficulty::Medium)
    }
    pub fn hard(&self, duration: std::time::Duration) -> Workout {
        self.difficulty(duration, Difficulty::Hard)
    }
    pub fn elite(&self, duration: std::time::Duration) -> Workout {
        self.difficulty(duration, Difficulty::Elite)
    }
}
