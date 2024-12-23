use crate::defaults;
use crate::duration::DurationExt;
use crate::mobiletimer;
use crate::routes;
use crate::signal;
use crate::signal::{SoundSignal, TimerState};
use crate::sound::Sound;
use defaults::default_sequences;
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Clone)]
pub struct Global {
    pub timer: dioxus::signals::Signal<mobiletimer::MobileTimer>,
    pub state: dioxus::signals::Signal<Rc<RefCell<TimerState>>>,
    pub bell: SoundSignal,
    pub beep: SoundSignal,
}

impl Global {
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

        let mut timer = use_signal(|| {
            mobiletimer::MobileTimer::new(std::time::Duration::from_secs(prepare), sequence)
        });

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

#[component]
pub fn MobileHome() -> Element {
    let sequences = default_sequences(
        &SoundSignal::none(),
        &SoundSignal::none(),
        &SoundSignal::none(),
    );
    rsx! {
        ul {
            for sequence in sequences.iter() {
                li {
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
            class: "flex text-2xl justify-center",
            to: routes::Route::WebHome {
                prepare: defaults::PREPARE,
                muted: false,
                sequence: "".to_string(),
            },
            {"Home"}
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

    rsx! {
        signal::Sounds { bell: global.bell, beep: global.beep }
        MobileControls {}
        div { id: "timer", class: "flex justify-evenly text-3xl p-2",
            button {
                id: "current_workout",
                onclick: move |_| global.timer.with_mut(|t| t.restart_workout()),
                {global.timer.read().label()}
                "(♻)"
            }
            label { id: "counter", {global.timer.read().left().to_string()} }
        }
        if !global.timer.read().sequence().is_empty() {
            ul { id: "sequence", class: "info flex-none p-2",
                for (index , item) in global.timer.read().sequence().iter().enumerate() {
                    li {
                        class: "text-nowrap",
                        class: if global.timer.read().sequence().index() == Some(index) { "text-red-600" } else { "" },
                        span { class: "text-sm", "{item}" }
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
                prepare: defaults::PREPARE,
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
                {defaults::RESTART_SEQUENCE}
            }
            button {
                id: "previous_item",
                class: "rounded-full text-3xl",
                onclick: move |_| global.timer.with_mut(|t| t.manual_previous()),
                {defaults::PREVIOUS_ITEM}
            }
            button {
                id: "next_item",
                class: "rounded-full text-3xl",
                onclick: move |_| global.timer.with_mut(|t| t.manual_next()),
                {defaults::NEXT_ITEM}
            }
            if global.timer.read().sequence().shufflable() {
                button {
                    id: "randomize",
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.shuffle()),
                    {defaults::RANDOMIZE}
                }
            }
            if !global.timer.read().sequence().signal().sound().is_silent() {
                button {
                    id: "toggle_signal",
                    class: "text-3xl",
                    onclick: move |_| global.state.with_mut(|s| s.borrow_mut().toggle()),
                    {global.state.read().borrow().next().to_string()}
                }
                button {
                    id: "emit_signal",
                    class: "text-3xl",
                    onclick: move |_| { global.timer.with(|t| { t.always_ring() }) },
                    {defaults::SIGNAL}
                }
            }
        }
    }
}
