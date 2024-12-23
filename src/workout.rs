use crate::duration::DurationExt;
use crate::item::{Item, REST};
use crate::sequence::Sequence;
use crate::signal::SoundSignal;
use crate::stopwatch::Stopwatch;
use crate::tag::{Difficulty, Tags};
use bon::Builder;
use derive_more::{Deref, DerefMut};

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash, Deref, DerefMut, Builder)]
pub struct Workout {
    item: Item,
    #[deref]
    #[deref_mut]
    #[builder(into)]
    stopwatch: Stopwatch,
    difficulty: Option<Difficulty>,
}

impl std::fmt::Display for Workout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} : {}",
            self.item,
            self.stopwatch.duration().to_string()
        )?;

        if let Some(joined_tags) = self.item.joined_tags() {
            write!(f, " ({joined_tags})")?;
        }
        Ok(())
    }
}

impl Workout {
    pub fn sequence(&self, signal: &SoundSignal) -> Sequence {
        Sequence::builder()
            .name(format!(
                "{} ({})",
                self.name(),
                self.stopwatch.duration().to_string()
            ))
            .description(self.description().clone().unwrap_or_default())
            .signal(signal.clone())
            .workouts(vec![self.clone()])
            .build()
    }
    pub fn rest(duration: std::time::Duration) -> Self {
        REST.workout(duration)
    }
    pub fn stopwatch(&self) -> &Stopwatch {
        &self.stopwatch
    }
    pub fn stopwatch_mut(&mut self) -> &mut Stopwatch {
        &mut self.stopwatch
    }
    pub fn is_rest(&self) -> bool {
        self.item.is_rest()
    }
    pub fn name(&self) -> &str {
        self.item.name()
    }
    pub fn tags(&self) -> &Tags {
        self.item.tags()
    }
    pub fn description(&self) -> &Option<String> {
        self.item.description()
    }
}
