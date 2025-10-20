use crate::duration::DurationExt;
use crate::item::Item;
use crate::item_list::ItemList;
use crate::sequence::Sequence;
use crate::sound::Sound;
use crate::stopwatch::Stopwatch;
use crate::tag::Difficulty;
use bon::Builder;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(
    Default, Clone, Debug, Eq, PartialEq, Hash, Deref, DerefMut, Builder, Serialize, Deserialize,
)]
pub struct Workout {
    item: Item,
    #[deref]
    #[deref_mut]
    #[builder(into)]
    #[serde(rename = "duration")]
    stopwatch: Stopwatch,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    difficulty: Option<Difficulty>,
}

impl std::fmt::Display for Workout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(difficulty) = self.difficulty {
            write!(f, "{} ", difficulty.icon())?;
        }
        write!(
            f,
            "{} : {}",
            self.item,
            self.stopwatch.duration().to_string()
        )?;

        if !self.is_rest() {
            if let Some(joined_tags) = self.item.joined_tags() {
                write!(f, " ({joined_tags})")?;
            }
        }
        Ok(())
    }
}

impl Workout {
    pub fn sequence(&self, sound: &Sound) -> Sequence {
        Sequence::builder()
            .name(format!(
                "{} ({})",
                self.item.name(),
                self.stopwatch.duration().to_string()
            ))
            .icon(self.item.icon().unwrap_or_default())
            .description(self.item.description().clone().unwrap_or_default())
            .sound(sound.clone())
            .workouts(vec![self.clone()])
            .build()
    }
    pub fn rest(duration: std::time::Duration) -> Self {
        ItemList::Rest.workout(duration)
    }
    pub fn stopwatch(&self) -> &Stopwatch {
        &self.stopwatch
    }
    pub fn is_rest(&self) -> bool {
        self.item.is_rest()
    }
    pub fn item(&self) -> &Item {
        &self.item
    }
    pub fn description(&self) -> &Option<String> {
        &self.item.description()
    }
}
