use crate::duration::DurationExt;
use crate::exercises::Exercises;
use crate::indexedvec::IndexedVec;
use crate::sound::Sound;
use crate::stopwatch::Stopwatch;
use crate::tag::{Difficulty, Tag};
use crate::workout::Workout;
use bon::{bon, Builder};
use derive_more::{Deref, DerefMut, Display};
use dioxus::logger::tracing::info;
use itertools::Itertools;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::collections::HashSet;
use std::ops::Not;
use std::sync::{LazyLock, RwLock};

pub static SEQUENCES: LazyLock<RwLock<HashSet<Sequence>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

// pub fn all_sequences() -> Vec<Sequence> {
//     SEQUENCES
//         .read()
//         .unwrap()
//         .iter()
//         .cloned()
//         .sorted_by(|a, b| a.to_string().cmp(&b.to_string()))
//         .collect_vec()
// }

pub fn all_sequences() -> Vec<Sequence> {
    crate::defaults::default_sequences()
}

#[derive(
    Display,
    Debug,
    Default,
    PartialEq,
    Eq,
    Clone,
    Deref,
    DerefMut,
    Builder,
    Serialize,
    Deserialize,
    Hash,
)]
#[display("{}{name}", icon.unwrap_or('‎'))]
pub struct Sequence {
    #[builder(into)]
    name: String,
    #[builder(into)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[deref]
    #[deref_mut]
    #[builder(into)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    workouts: IndexedVec<Workout>,
    sound: Sound,
    #[serde(skip_serializing_if = "Option::is_none")]
    rest: Option<std::time::Duration>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    shuffleable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    difficulty: Option<Difficulty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    icon: Option<char>,
}

type Rounds = usize;
pub static ROUNDS: Rounds = 1;

#[bon]
impl Sequence {
    pub fn register(self) -> Self {
        info!(
            "registering sequence: {}",
            serde_json::to_string(&self).unwrap()
        );
        SEQUENCES.write().unwrap().insert(self.clone());
        for workout in self.workouts.iter() {
            workout.register();
        }
        self
    }

    #[builder]
    pub fn random(
        name: &str,
        description: Option<&str>,
        workouts: &[Workout],
        rest: std::time::Duration,
        sound: &Sound,
        difficulty: Option<Difficulty>,
        icon: Option<char>,
    ) -> Self {
        let mut workouts = workouts.to_vec();
        workouts.shuffle(&mut rand::thread_rng());

        let workouts = itertools::intersperse(workouts, Workout::rest(rest)).collect_vec();
        Self {
            name: name.into(),
            description: description.map(str::to_string),
            workouts: workouts.into(),
            sound: sound.clone(),
            rest: Some(rest),
            shuffleable: true,
            difficulty,
            icon,
        }
    }
    #[builder]
    pub fn simple(
        name: &str,
        description: Option<&str>,
        workouts: &[Workout],
        sound: &Sound,
        difficulty: Option<Difficulty>,
        icon: Option<char>,
    ) -> Self {
        let total = std::time::Duration::from_secs(
            workouts
                .iter()
                .map(|workout| workout.duration().as_secs())
                .sum(),
        );
        Self {
            name: format!("{name} ({} total)", total.to_string()),
            description: description.map(str::to_string),
            workouts: IndexedVec::from(workouts),
            sound: sound.clone(),
            rest: None,
            shuffleable: false,
            difficulty,
            icon,
        }
    }
    #[builder]
    #[allow(clippy::too_many_arguments)]
    pub fn repeat(
        name: &str,
        description: Option<&str>,
        exercises: Exercises,
        workout: std::time::Duration,
        rounds: Rounds,
        rest: std::time::Duration,
        sound: &Sound,
        difficulty: Option<Difficulty>,
        icon: Option<char>,
    ) -> Self {
        #[allow(unstable_name_collisions)]
        let workouts = std::iter::repeat(exercises.workouts(workout))
            .take(rounds)
            .intersperse(vec![Workout::rest(rest)])
            .flatten()
            .collect_vec();
        Self {
            name: format!("{name} ({}s rest)", rest.as_secs()),
            description: description.map(str::to_string),
            workouts: workouts.into(),
            sound: sound.clone(),
            rest: None,
            shuffleable: false,
            difficulty,
            icon,
        }
    }
    #[builder]
    pub fn rounds(
        name: &str,
        description: Option<&str>,
        rounds: Rounds,
        workout: Workout,
        rest: std::time::Duration,
        sound: &Sound,
        difficulty: Option<Difficulty>,
        icon: Option<char>,
    ) -> Self {
        #[allow(unstable_name_collisions)]
        let workouts = std::iter::repeat(workout)
            .take(rounds)
            .intersperse(Workout::rest(rest))
            .collect_vec();
        Self {
            name: format!("{name} ({}s rest)", rest.as_secs()),
            description: description.map(str::to_string),
            workouts: workouts.into(),
            sound: sound.clone(),
            rest: None,
            shuffleable: false,
            difficulty,
            icon,
        }
    }
    pub fn slug(&self) -> String {
        slugify(&self.name)
    }
    pub fn goto_previous(&mut self) -> Option<&mut Workout> {
        info!("sequence: goto previous");
        if self.workouts.is_empty() {
            info!("sequence: workouts is empty, no previous");
            return None;
        }

        #[allow(clippy::manual_inspect)]
        self.workouts.previous_mut().map(|p| {
            p.reset();
            p
        })
    }
    pub fn manual_next(&mut self) -> Option<&mut Workout> {
        info!("sequence: manual next");
        if self.workouts.is_empty() {
            info!("sequence: workouts is empty, no next");
        } else if !self.workouts.last() {
            self.workouts.next_mut();
            self.reset_workout();
        } else {
            self.workouts.set_index(0);
            self.reset_workout();
        }
        self.get_mut()
    }
    pub fn auto_next(&mut self) -> Option<&mut Workout> {
        info!("sequence: auto next");
        if self.workouts.is_empty() {
            info!("sequence: workouts is empty, no next");
            return None;
        }
        let workout = self.workouts.next_mut()?;
        workout.reset();
        Some(workout)
    }
    pub fn stopwatch(&mut self) -> Option<&Stopwatch> {
        self.workouts.get().map(|i| i.stopwatch())
    }
    pub fn decrement(&mut self) -> bool {
        self.workouts.get_mut().map_or(false, |i| i.decrement())
    }
    pub fn last_seconds(&self) -> bool {
        self.workouts.get().map_or(false, |i| i.last_seconds())
    }
    pub fn reset_workout(&mut self) {
        info!("sequence: reset current workout");
        if let Some(i) = self.workouts.get_mut() {
            i.reset();
        }
    }
    pub fn reset(&mut self) {
        info!("sequence: reset all workouts");
        self.workouts.reset();
        self.workouts.apply(|workout| {
            workout.reset();
        });
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn total(&self) -> std::time::Duration {
        std::time::Duration::from_secs(
            self.iter()
                .map(|workout| workout.duration().as_secs())
                .sum(),
        )
    }
    pub fn left_total(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.iter().map(|workout| workout.left().as_secs()).sum())
    }
    pub fn workout_total(&self) -> std::time::Duration {
        std::time::Duration::from_secs(
            self.iter()
                .filter(|workout| !workout.is_rest())
                .map(|workout| workout.duration().as_secs())
                .sum(),
        )
    }
    pub fn rest_total(&self) -> std::time::Duration {
        std::time::Duration::from_secs(
            self.iter()
                .filter(|workout| workout.is_rest())
                .map(|workout| workout.duration().as_secs())
                .sum(),
        )
    }
    pub fn tags(&self) -> Vec<Tag> {
        self.workouts
            .iter()
            .filter(|workout| !workout.is_rest())
            .flat_map(|workout| workout.item().tags().clone())
            .unique()
            .collect_vec()
    }
    pub fn sound(&self) -> &Sound {
        &self.sound
    }
    pub fn shuffle(&mut self) {
        if self.shuffleable {
            // if it was interspersed with rest, rebuild sequence
            if let Some(rest) = self.rest {
                let rest = Workout::rest(rest);
                self.workouts.retain(&rest);
                let workouts = itertools::intersperse(self.workouts.shuffled(), rest).collect_vec();
                self.workouts = workouts.into();
            } else {
                self.workouts.shuffle()
            }
        }
    }
    pub fn shufflable(&self) -> bool {
        self.shuffleable
    }
}

#[test]
fn sequence_simple_tests() {
    use crate::duration::SECOND;
    use crate::item::Item;
    let warm_up = Item::builder().name("warm up").build().workout(3 * SECOND);
    let workout = Item::builder().name("workout").build().workout(6 * SECOND);
    let mut simple = Sequence::simple()
        .name("simple sequence")
        .workouts(&[warm_up.clone(), workout.clone()])
        .sound(&Sound::Silent)
        .call();

    assert!(simple.auto_next().is_some());
    assert_eq!(simple.get(), Some(&warm_up));

    assert!(simple.auto_next().is_some());
    assert_eq!(simple.get(), Some(&workout));

    assert!(simple.auto_next().is_none());
    assert_eq!(simple.get(), None);

    simple.manual_next();
    assert_eq!(simple.get(), Some(&warm_up));

    simple.manual_next();
    assert_eq!(simple.get(), Some(&workout));

    assert!(simple.auto_next().is_none());
    assert_eq!(simple.get(), None);

    simple.manual_next();
    assert_eq!(simple.get(), Some(&warm_up));

    simple.reset();
    assert_eq!(simple.get(), None);
}
