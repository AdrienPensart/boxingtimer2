use crate::audio::Sounds;
use crate::timer;
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

#[component]
pub fn MobileHome() -> Element {
    let sequences = SEQUENCES.as_slice();
    rsx! {
        ul { id: "sequences",
            for sequence in sequences.iter() {
                li { id: format!("sequence_{}", sequence.slug()),
                    Link {
                        to: routes::Route::MobileTimer {
                            slug: sequence.slug(),
                        },
                        title: format!("Start timer for {}", sequence.name()),
                        {sequence.to_string()}
                    }
                }
            }
        }
    }
}

#[component]
pub fn MobileTimer(slug: String) -> Element {
    let Some(global) = Global::new(false, 10, &slug) else {
        return rsx! { "unknown sequence" };
    };
    if global.timer.read().sequence().is_empty() {
        return rsx! { "empty sequence" };
    }
    let mut global = use_context_provider(|| global);
    let timer = global.timer.read();
    rsx! {
        Sounds {}
        div { id: "timer", class: "grid gap-4 grid-cols-1 text-3xl p-2",
            MobileControls {}
            div { class: "flex items-center justify-center",
                button {
                    id: "current_workout",
                    title: "Restart workout",
                    onclick: move |_| global.timer.with_mut(super::timer::Timer::restart_workout),
                    {timer.label()}
                }
            }
            div { class: "flex items-center justify-center",
                span { id: "counter", title: "Time left", {global.timer.read().left().to_string()} }
            }
            if let Some(next_workout) = global.timer.read().sequence().next_workout() {
                div { class: "flex items-center justify-center",
                    span { id: "next_exercise",
                        {"Next: "}
                        {next_workout.item().name()}
                    }
                }
            } else {
                span { class: "flex items-center justify-center", "END" }
            }
            div { class: "flex items-center justify-center",
                Link {
                    id: "exercises_link",
                    title: "See exercises in this sequence",
                    to: routes::Route::SequenceHome {
                        slug: global.timer.read().sequence().slug(),
                    },
                    {"See exercises"}
                }
            }
            div { class: "flex items-center justify-center",
                Link {
                    id: "home_link",
                    title: "Go to sequence list",
                    to: routes::Route::SequencesHome {},
                    {"Home"}
                }
            }
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
                title: global.timer.read().status().next_title(),
                onclick: move |_| global.timer.with_mut(super::timer::Timer::toggle),
                {global.timer.read().status().next().to_string()}
            }
            button {
                id: "restart_sequence",
                class: "rounded-full text-3xl",
                title: "Restart sequence",
                onclick: move |_| global.timer.with_mut(super::timer::Timer::restart_sequence),
                {RESTART_SEQUENCE}
            }
            button {
                id: "previous_workout",
                class: "rounded-full text-3xl",
                title: "Previous workout",
                onclick: move |_| global.timer.with_mut(super::timer::Timer::manual_previous),
                {PREVIOUS_ITEM}
            }
            button {
                id: "next_workout",
                class: "rounded-full text-3xl",
                title: "Next workout",
                onclick: move |_| global.timer.with_mut(super::timer::Timer::manual_next),
                {NEXT_ITEM}
            }
            if global.timer.read().sequence().shufflable() {
                button {
                    id: "randomize",
                    class: "rounded-full text-3xl",
                    title: "Shuffle sequence",
                    onclick: move |_| global.timer.with_mut(super::timer::Timer::shuffle),
                    {RANDOMIZE}
                }
            }
            if !global.timer.read().sequence().sound().is_silent() {
                button {
                    id: "toggle_signal",
                    class: "text-3xl",
                    title: global.sound_signal.read().state().next_title(),
                    onclick: move |_| global.sound_signal.with_mut(super::signal::SoundSignal::toggle),
                    input {
                        r#type: "checkbox",
                        checked: global.sound_signal.read().enabled(),
                        id: "mute_or_unmute",
                    }
                    {global.sound_signal.read().next().to_string()}
                }
                button {
                    id: "emit_signal",
                    class: "text-3xl",
                    title: "Emit signal sound",
                    onclick: move |_| { global.timer.with(|t| { t.ring() }) },
                    {SIGNAL}
                }
            }
        }
    }
}
