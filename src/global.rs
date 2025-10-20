use crate::signal::SoundSignal;
use crate::timer;
use dioxus::prelude::*;
use sport::defaults::{DEFAULT_INTERVAL, PREPARE, SEQUENCES};

#[derive(Clone)]
pub struct Global {
    pub timer: dioxus::signals::Signal<timer::Timer>,
    pub sound_signal: dioxus::signals::Signal<SoundSignal>,
}

impl Global {
    pub fn new(muted: bool, prepare: u64, sequence: &str) -> Option<Self> {
        let prepare = if prepare == 0 { PREPARE } else { prepare };
        let sequence = SEQUENCES.iter().find(|s| s.slug() == sequence)?;
        let sound_signal = SoundSignal::from_muted(muted);
        let mut timer = use_signal(|| {
            timer::Timer::new(
                std::time::Duration::from_secs(prepare),
                sequence,
                &sound_signal,
            )
        });

        let _tick = use_resource(move || async move {
            loop {
                gloo::timers::future::TimeoutFuture::new(DEFAULT_INTERVAL).await;
                if timer.write().tick() {
                    gloo::timers::future::TimeoutFuture::new(DEFAULT_INTERVAL).await;
                }
            }
        });

        Some(Self {
            sound_signal: use_signal(|| sound_signal),
            timer,
        })
    }
}
