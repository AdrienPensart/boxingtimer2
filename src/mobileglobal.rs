use crate::defaults;
use crate::mobiletimer::MobileTimer;
use crate::signal::{Signal, State};
use crate::sound::Sound;
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Clone)]
pub struct MobileGlobal {
    pub timer: dioxus_signals::Signal<MobileTimer>,
    pub state: dioxus_signals::Signal<Rc<RefCell<State>>>,
    pub bell: Signal,
    pub beep: Signal,
}

impl MobileGlobal {
    pub fn new(muted: bool, prepare: u64, sequence: String) -> Option<Self> {
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
        let sequences = &defaults::default_sequences(&bell, &beep, &silent);
        let sequence = sequences.iter().find(|s| s.slug() == sequence)?;

        let mut timer =
            use_signal(|| MobileTimer::new(std::time::Duration::from_secs(prepare), sequence));

        let _tick = use_resource(move || async move {
            loop {
                gloo::timers::future::TimeoutFuture::new(defaults::DEFAULT_INTERVAL).await;
                if timer.write().tick() {
                    gloo::timers::future::TimeoutFuture::new(defaults::DEFAULT_INTERVAL).await;
                }
            }
        });

        Some(Self {
            state: use_signal(|| state.clone()),
            bell,
            beep,
            timer,
        })
    }
}
