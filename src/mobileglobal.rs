use crate::defaults;
use crate::mobiletimer::MobileTimer;
use crate::signal::{SoundSignal, TimerState};
use crate::sound::Sound;
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Clone)]
pub struct MobileGlobal {
    pub timer: dioxus::signals::Signal<MobileTimer>,
    pub state: dioxus::signals::Signal<Rc<RefCell<TimerState>>>,
    pub bell: SoundSignal,
    pub beep: SoundSignal,
}

impl MobileGlobal {
    pub fn new(muted: bool, prepare: u64, sequence: String) -> Option<Self> {
        let state = if muted {
            TimerState::Disabled
        } else {
            TimerState::Enabled
        };
        let prepare = if prepare == 0 {
            defaults::PREPARE
        } else {
            prepare
        };
        let state = Rc::new(RefCell::new(state));

        let silent = SoundSignal::new(Sound::Silent, state.clone());
        let bell = SoundSignal::new(Sound::Bell, state.clone());
        let beep = SoundSignal::new(Sound::Beep, state.clone());
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
