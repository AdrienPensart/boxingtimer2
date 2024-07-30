use crate::duration::Duration;
use crate::errors::BoxingTimerErrorKind;
use crate::helpers;
use crate::status::Status;
// use crate::rounds::Rounds;
use crate::sequence::Sequence;
use crate::stopwatch::Stopwatch;
use lenient_bool::LenientBool;

pub const DEFAULT_INTERVAL: u64 = 1000;

#[derive(Debug, Default)]
pub struct BoxingTimer {
    /// Is the timer running ?
    status: Status,
    /// Current state of training
    // state: State,
    /// Left seconds in current round
    stopwatch: Stopwatch,
    /// Current exercise
    current_item: Option<usize>,
    /// Full sequence
    sequence: Sequence,
}

impl BoxingTimer {
    pub fn new() -> Result<Self, BoxingTimerErrorKind> {
        let rounds_param = helpers::get_param_or("rounds", 12);
        let prepare_param = helpers::get_param_or("prepare", 5);
        let fight_param = helpers::get_param_or("fight", 180);
        let rest_param = helpers::get_param_or("rest", 60);
        let start_param = helpers::get_param_or::<LenientBool>("start", LenientBool(false));
        let status = if start_param.into() {
            Status::Running
        } else {
            Status::Paused
        };

        let prepare = Duration::from_secs(prepare_param);
        let rest = Duration::from_secs(rest_param);
        let fight = Duration::from_secs(fight_param);
        let rounds = rounds_param as usize;
        let sequence = Sequence::boxing_sequence(rounds as usize, prepare, fight, rest)?;
        let stopwatch = Stopwatch::new(Duration::default());
        Ok(Self {
            status,
            stopwatch,
            sequence,
            ..Self::default()
        })
    }
    pub fn current_item(&self) -> Option<usize> {
        self.current_item
    }
    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }
    pub fn stopwatch(&self) -> Stopwatch {
        self.stopwatch
    }
    pub fn reset_beginning(&mut self) {
        log::info!("reseting beginning");
        if !self.sequence.is_empty() {
            self.current_item = Some(0);
            self.stopwatch.set(*self.sequence[0].duration());
        }
    }
    pub fn reset_current(&mut self) {
        log::info!("reseting current");
        if let Some(current_item) = self.current_item {
            self.stopwatch.set(*self.sequence[current_item].duration());
        }
    }
    pub fn tick(&mut self) {
        let Some(current_item) = self.current_item else {
            return;
        };
        if self.status == Status::Paused {
            return;
        }
        if self.stopwatch.decrement() {
            return;
        }
        if current_item == self.sequence.len() - 1 {
            self.status = Status::Paused;
            self.current_item = None;
            return;
        }
        self.current_item = Some(current_item + 1);
        self.stopwatch
            .set(*self.sequence[current_item + 1].duration())
    }
    pub fn goto_next(&mut self) {
        log::info!("goto next");
        if let Some(current_item) = self.current_item {
            self.current_item = Some(current_item + 1);
            self.stopwatch
                .set(*self.sequence[current_item + 1].duration());
            return;
        }
        if self.current_item.is_none() && !self.sequence.is_empty() {
            self.current_item = Some(0);
            self.stopwatch.set(*self.sequence[0].duration())
        }
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn toggle(&mut self) {
        self.status.toggle();
        if self.status == Status::Running
            && self.current_item.is_none()
            && !self.sequence.is_empty()
        {
            self.current_item = Some(0);
            self.stopwatch.set(*self.sequence[0].duration())
        }
    }
    pub fn next_status(&self) -> Status {
        self.status.next()
    }
}
