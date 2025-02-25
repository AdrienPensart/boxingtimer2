use crate::audio::Sounds;
use crate::mobiletimer;
use crate::routes;
use crate::signal::SoundSignal;
use dioxus::prelude::*;
use sport::defaults::{
    DEFAULT_INTERVAL, NEXT_ITEM, PREPARE, PREVIOUS_ITEM, RANDOMIZE, RESTART_SEQUENCE, SEQUENCES,
    SIGNAL,
};
use sport::duration::DurationExt;

#[derive(Clone)]
pub struct Global {
    pub timer: dioxus::signals::Signal<mobiletimer::MobileTimer>,
    pub sound_signal: dioxus::signals::Signal<SoundSignal>,
}

impl Global {
    pub fn new(muted: bool, prepare: u64, sequence: String) -> Option<Self> {
        let prepare = if prepare == 0 { PREPARE } else { prepare };
        let sequence = SEQUENCES.iter().find(|s| s.slug() == sequence)?;
        let sound_signal = SoundSignal::from_muted(muted);
        let mut timer = use_signal(|| {
            mobiletimer::MobileTimer::new(
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

#[component]
pub fn MobileHome() -> Element {
    let sequences = SEQUENCES.as_slice();
    rsx! {
        ul { id: "sequences",
            for sequence in sequences.iter() {
                li { id: format!("sequence_{}", sequence.slug()),
                    Link {
                        to: routes::Route::MobileTimer {
                            sequence: sequence.slug(),
                        },
                        {sequence.to_string()}
                    }
                }
            }
        }
        Link {
            id: "web_home",
            class: "flex text-2xl justify-center",
            to: routes::Route::WebHome {
                prepare: PREPARE,
                muted: false,
                sequence: "".to_string(),
            },
            {"Web Home"}
        }
    }
}

#[component]
pub fn MobileTimer(sequence: String) -> Element {
    let Some(global) = Global::new(false, 10, sequence) else {
        return rsx! { "Unknown sequence" };
    };
    if global.timer.read().sequence().is_empty() {
        return rsx! { "Empty sequence" };
    }
    let mut global = use_context_provider(|| global);
    let timer = global.timer.read();
    rsx! {
        Sounds {}
        MobileControls {}
        div { id: "timer", class: "flex justify-evenly text-3xl p-2",
            button {
                id: "current_workout",
                onclick: move |_| global.timer.with_mut(|t| t.restart_workout()),
                {timer.label()}
                "(♻)"
            }
            label { id: "counter", {global.timer.read().left().to_string()} }
        }
        if !global.timer.read().sequence().is_empty() {
            ul { id: "sequence", class: "info flex-none p-2",
                for (index , workout) in global.timer.read().sequence().iter().enumerate() {
                    li {
                        class: "text-nowrap",
                        class: if global.timer.read().sequence().index() == Some(index) { "text-red-600" } else { "" },
                        span { class: "text-sm", "{workout}" }
                    }
                }
            }
        }
        Link {
            id: "mobile_home",
            class: "flex text-2xl justify-center",
            to: routes::Route::MobileHome {},
            {"Mobile Home"}
        }
        Link {
            id: "web_home",
            class: "flex text-2xl justify-center",
            to: routes::Route::WebHome {
                prepare: PREPARE,
                muted: false,
                sequence: "".to_string(),
            },
            {"Web Home"}
        }
    }
}

#[component]
pub fn MobileControls() -> Element {
    let mut global = use_context::<Global>();
    rsx! {
        div { id: "controls", class: "flex justify-evenly p-2",
            button {
                id: "toggle_timer",
                class: "rounded-full text-3xl",
                onclick: move |_| global.timer.with_mut(|t| t.toggle()),
                {global.timer.read().next_status().to_string()}
            }
            button {
                id: "restart_sequence",
                class: "rounded-full text-3xl",
                onclick: move |_| global.timer.with_mut(|t| t.restart_sequence()),
                {RESTART_SEQUENCE}
            }
            button {
                id: "previous_workout",
                class: "rounded-full text-3xl",
                onclick: move |_| global.timer.with_mut(|t| t.manual_previous()),
                {PREVIOUS_ITEM}
            }
            button {
                id: "next_workout",
                class: "rounded-full text-3xl",
                onclick: move |_| global.timer.with_mut(|t| t.manual_next()),
                {NEXT_ITEM}
            }
            if global.timer.read().sequence().shufflable() {
                button {
                    id: "randomize",
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.shuffle()),
                    {RANDOMIZE}
                }
            }
            if !global.timer.read().sequence().sound().is_silent() {
                button {
                    id: "toggle_signal",
                    class: "text-3xl",
                    onclick: move |_| global.sound_signal.with_mut(|s| s.toggle()),
                    {global.sound_signal.read().next().to_string()}
                }
                button {
                    id: "emit_signal",
                    class: "text-3xl",
                    onclick: move |_| { global.timer.with(|t| { t.always_ring() }) },
                    {SIGNAL}
                }
            }
        }
    }
}
