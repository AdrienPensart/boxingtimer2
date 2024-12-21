use crate::defaults;
use crate::sequence::Sequence;
use crate::signal::{Signal, State};
use crate::sound::Sound;

use crate::timer::Timer;
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Clone)]
pub struct Global {
    pub timer: dioxus_signals::Signal<Timer>,
    pub state: dioxus_signals::Signal<Rc<RefCell<State>>>,
    pub sequences: Vec<Sequence>,
    pub bell: Signal,
    pub beep: Signal,
}

impl Global {
    pub fn new(muted: bool, prepare: u64, sequence: Option<String>) -> Self {
        let state = if muted {
            State::Disabled
        } else {
            State::Enabled
        };
        let prepare = if prepare == 0 {
            defaults::PREPARE
        } else {
            prepare
        };
        let state = Rc::new(RefCell::new(state));

        let silent = Signal::new(Sound::Silent, state.clone());
        let bell = Signal::new(Sound::Bell, state.clone());
        let beep = Signal::new(Sound::Beep, state.clone());
        let sequences = defaults::default_sequences(&bell, &beep, &silent);

        let mut timer = use_signal(|| {
            let mut timer = Timer::new(std::time::Duration::from_secs(prepare), &sequences);
            if let Some(sequence) = sequence {
                timer.set_sequence_by_slug(&sequence)
            }
            timer
        });

        let _tick = use_resource(move || async move {
            loop {
                gloo::timers::future::TimeoutFuture::new(defaults::DEFAULT_INTERVAL).await;
                if timer.write().tick() {
                    gloo::timers::future::TimeoutFuture::new(defaults::DEFAULT_INTERVAL).await;
                }
            }
        });

        Self {
            state: use_signal(|| state.clone()),
            sequences,
            bell,
            beep,
            timer,
        }
    }
}
