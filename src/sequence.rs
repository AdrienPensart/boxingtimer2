use crate::duration::Duration;
use crate::errors::BoxingTimerErrorKind;
use crate::item::{Boxing, Item, Prepare, Rest};
use derive_more::{Deref, Index, IntoIterator};
use derive_new::new;

#[derive(new, Default, PartialEq, Eq, Clone, Debug, IntoIterator, Index, Deref)]
pub struct Sequence(Vec<Item>);

impl Sequence {
    pub fn boxing_sequence(
        rounds: usize,
        prepare: Duration,
        workout: Duration,
        rest: Duration,
    ) -> Result<Self, BoxingTimerErrorKind> {
        let prepare_item = Prepare(prepare)?;
        let boxing_item = Boxing(workout)?;
        let rest_item = Rest(rest)?;
        let rounds_items = vec![boxing_item; rounds];
        Ok(Self(
            itertools::chain(
                std::iter::once(prepare_item),
                itertools::intersperse(rounds_items, rest_item),
            )
            .collect(),
        ))
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
