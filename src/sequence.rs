use crate::duration::Duration;
use crate::errors::TimerErrorKind;
use crate::item::{Boxing, Item, Prepare, Rest, Workout};
use derive_more::{Deref, Index, IntoIterator};
use derive_new::new;

#[derive(new, Default, PartialEq, Eq, Clone, Debug, IntoIterator, Index, Deref)]
pub struct Sequence {
    #[deref]
    #[index]
    #[into_iterator]
    items: Vec<Item>,
    restart: bool,
    name: String,
}

impl Sequence {
    pub fn hiit_sequence(workout: Duration, rest: Duration) -> Result<Self, TimerErrorKind> {
        Ok(Self {
            name: "HiiT".into(),
            items: vec![Rest(rest)?, Workout(workout)?],
            restart: true,
        })
    }
    pub fn boxing_sequence(
        prepare: Duration,
        rounds: usize,
        workout: Duration,
        rest: Duration,
    ) -> Result<Self, TimerErrorKind> {
        let prepare_item = Prepare(prepare)?;
        let boxing_item = Boxing(workout)?;
        let rest_item = Rest(rest)?;
        let rounds_items = vec![boxing_item; rounds];
        Ok(Self {
            name: "Boxing".into(),
            items: itertools::chain(
                std::iter::once(prepare_item),
                itertools::intersperse(rounds_items, rest_item),
            )
            .collect(),
            restart: false,
        })
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn restart(&self) -> bool {
        self.restart
    }
    pub fn total(&self) -> Duration {
        Duration::from_secs(
            self.iter()
                .filter(|i| !i.prepare())
                .map(|i| i.duration().as_secs())
                .sum(),
        )
    }
    pub fn workout_total(&self) -> Duration {
        Duration::from_secs(
            self.iter()
                .filter(|i| !i.waiting() && !i.prepare())
                .map(|i| i.duration().as_secs())
                .sum(),
        )
    }
    pub fn waiting_total(&self) -> Duration {
        Duration::from_secs(
            self.iter()
                .filter(|i| i.waiting() && !i.prepare())
                .map(|i| i.duration().as_secs())
                .sum(),
        )
    }
}
