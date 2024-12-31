use crate::tag::{Difficulty, Tag, Tags};
use crate::workout::Workout;
use bon::Builder;
use derive_more::Display;
use dioxus::logger::tracing::info;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, RwLock};

pub static ITEMS: LazyLock<RwLock<HashSet<Item>>> = LazyLock::new(|| RwLock::new(HashSet::new()));

pub fn all_items() -> Vec<Item> {
    ITEMS
        .read()
        .unwrap()
        .iter()
        .cloned()
        .sorted_by(|a, b| a.to_string().cmp(&b.to_string()))
        .collect_vec()
}

pub static TAG_TO_ITEMS: LazyLock<RwLock<HashMap<Tag, HashSet<Item>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn all_tags() -> Vec<Tag> {
    TAG_TO_ITEMS
        .read()
        .unwrap()
        .keys()
        .cloned()
        .sorted_by(|a, b| a.to_string().cmp(&b.to_string()))
        .collect_vec()
}

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
    pub fn register(self) -> Self {
        info!(
            "registering item: {}",
            serde_json::to_string(&self).unwrap()
        );
        for tag in self.tags.iter() {
            TAG_TO_ITEMS
                .write()
                .unwrap()
                .entry(*tag)
                .or_default()
                .insert(self.clone());
        }
        ITEMS.write().unwrap().insert(self.clone());
        self
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
