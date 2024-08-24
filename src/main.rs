#![allow(non_snake_case)]
pub mod beep;
pub mod difficulty;
pub mod duration;
pub mod errors;
pub mod indexedvec;
pub mod item;
pub mod sequence;
pub mod signal;
pub mod sound;
pub mod status;
pub mod stopwatch;
pub mod tag;
pub mod timer;
use crate::duration::DurationExt;
use crate::item::GenericItem;
use crate::item::{Easy, Prepare};
use crate::sequence::Sequence;
use crate::signal::{Signal, State};
use crate::sound::Sound;
use dioxus::prelude::*;
// use dioxus_logger::tracing::error;
use dioxus_logger::tracing::Level;
use manganis::mg;
use std::{cell::RefCell, rc::Rc};

const _: &str = mg!(file("assets/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/?:muted&:start&:prepare")]
    BoxingTimer {
        muted: bool,
        start: bool,
        prepare: u64,
    },
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn BoxingTimer(muted: bool, start: bool, prepare: u64) -> Element {
    let state = if muted {
        State::Disabled
    } else {
        State::Enabled
    };
    let state = Rc::new(RefCell::new(state));

    let silent = Signal::new(Sound::Silent, state.clone());
    let bell = Signal::new(Sound::Bell, state.clone());
    let beep = Signal::new(Sound::Beep, state.clone());

    let prepare = &std::time::Duration::from_secs(prepare);
    let warmup_boxing = Sequence::simple(
        "Warm Up",
        &[
            Prepare(prepare),
            // 1 minute
            Easy("Head rotation", &std::time::Duration::from_secs(20)),
            Easy("Shoulders rotation", &std::time::Duration::from_secs(20)),
            Easy("Arms rotation", &std::time::Duration::from_secs(20)),
            // 1 minute
            Easy("Elbows rotation", &std::time::Duration::from_secs(20)),
            Easy("Wrists rotation", &std::time::Duration::from_secs(20)),
            Easy("Hips rotation", &std::time::Duration::from_secs(20)),
            // 1 minute
            Easy("Knees rotation", &std::time::Duration::from_secs(20)),
            Easy("Feet rotation", &std::time::Duration::from_secs(20)),
            Easy("Heel raises", &std::time::Duration::from_secs(20)),
            // 1 minute
            Easy("Leg swings", &std::time::Duration::from_secs(20)),
            Easy("Side leg swings", &std::time::Duration::from_secs(20)),
            Easy("Single leg touch toes", &std::time::Duration::from_secs(20)),
            // 1 minute
            Easy("Butt kicks", &std::time::Duration::from_secs(30)),
            Easy("High knees", &std::time::Duration::from_secs(30)),
            // 1 minute
            Easy("Jumping jacks", &std::time::Duration::from_secs(30)),
            Easy("Mountain climbers", &std::time::Duration::from_secs(30)),
            // 1 minute
            Easy("Jump squats", &std::time::Duration::from_secs(30)),
            Easy("Push ups", &std::time::Duration::from_secs(30)),
            // 1 minute
            Easy("Speed steps", &std::time::Duration::from_secs(30)),
            Easy("Left/right jumps", &std::time::Duration::from_secs(30)),
            // 1 minute
            Easy("Alternate lunges", &std::time::Duration::from_secs(30)),
            Easy("Burpees", &std::time::Duration::from_secs(30)),
        ],
        &[tag::Tag::WarmUp],
        &silent,
    );

    let _3x2m_30s = Sequence::rounds(
        "3x2m (30s rest)",
        3,
        prepare,
        &GenericItem::Duration(120),
        &std::time::Duration::from_secs(30),
        &[tag::Tag::Boxing],
        &bell,
    );

    let _6x2m_30s = Sequence::rounds(
        "6x2m (30s rest)",
        6,
        prepare,
        &GenericItem::Duration(120),
        &std::time::Duration::from_secs(30),
        &[tag::Tag::Boxing],
        &bell,
    );

    let stamina_jab_cross_hook = Sequence::repeat(
        "1 | 2 | 1-2 | 1-2-3 (60s rest)",
        vec![
            "Jab (1)",
            "Cross (2)",
            "Jab | Cross (1-2)",
            "Jab | Cross | Hook (1-2-3)",
        ],
        prepare,
        &std::time::Duration::from_secs(30),
        4,
        &std::time::Duration::from_secs(60),
        &[tag::Tag::Boxing, tag::Tag::Stamina],
        &bell,
    );

    let stamina_jab_cross_hook_cross = Sequence::repeat(
        "1 | 1-2 | 1-2-3 | 1-2-3-2 (60s rest)",
        vec![
            "Jab (1)",
            "Jab | Cross (1-2)",
            "Jab | Cross | Hook (1-2-3)",
            "Jab | Cross | Hook | Cross (1-2-3-2)",
        ],
        prepare,
        &std::time::Duration::from_secs(30),
        4,
        &std::time::Duration::from_secs(60),
        &[tag::Tag::Boxing, tag::Tag::Stamina],
        &bell,
    );

    let hiit = Sequence::infinite(
        prepare,
        &GenericItem::Duration(20),
        &std::time::Duration::from_secs(10),
        &[tag::Tag::HiiT],
        &beep,
    );

    let _5mn = Sequence::workout("5mn", prepare, &GenericItem::Duration(5 * 60), &[], &bell);
    let _10mn = Sequence::workout("10mn", prepare, &GenericItem::Duration(10 * 60), &[], &bell);

    let mut state_signal = use_signal(|| state.clone());

    let mut timer = use_signal(|| {
        timer::Timer::new(&[
            warmup_boxing,
            _3x2m_30s,
            _6x2m_30s,
            stamina_jab_cross_hook,
            stamina_jab_cross_hook_cross,
            hiit,
            _5mn,
            _10mn,
        ])
    });
    let _tick = use_resource(move || async move {
        loop {
            gloo::timers::future::TimeoutFuture::new(timer::DEFAULT_INTERVAL).await;
            if timer.write().tick() {
                gloo::timers::future::TimeoutFuture::new(timer::DEFAULT_INTERVAL).await;
            }
        }
    });

    use_effect(move || {
        if start {
            timer.write().toggle();
        }
    });

    rsx! {
        div { class: "flex flex-wrap space-x-2",
            div { class: "",
                button {
                    class: "btn btn-primary rounded-full w-24 m-2",
                    onclick: move |_| timer.with_mut(|t| t.toggle()),
                    {timer.read().next_status().to_string()}
                }
                button {
                    class: "btn btn-primary rounded-full m-2",
                    onclick: move |_| timer.with_mut(|t| t.restart_sequence()),
                    "Restart sequence"
                }
                button {
                    class: "btn btn-primary rounded-full m-2",
                    onclick: move |_| timer.with_mut(|t| t.restart_item()),
                    "Restart current"
                }
            }
            div { class: "",
                button {
                    class: "btn btn-accent rounded-full m-2",
                    onclick: move |_| timer.with_mut(|t| t.manual_previous()),
                    "Previous"
                }
                button {
                    class: "btn btn-accent rounded-full m-2",
                    onclick: move |_| timer.with_mut(|t| t.manual_next()),
                    "Next"
                }
            }
            div { class: "",
                audio {
                    id: bell.to_string(),
                    src: bell.asset(),
                    preload: "auto",
                    autoplay: false
                }
                audio {
                    id: beep.to_string(),
                    src: beep.asset(),
                    preload: "auto",
                    autoplay: false
                }
                button {
                    class: "btn btn-secondary rounded-full w-24 m-2",
                    onclick: move |_| state_signal.with_mut(|s| s.borrow_mut().toggle()),
                    { state_signal.read().borrow().next_label() }
                }
                button {
                    class: "btn btn-secondary rounded-full m-2",
                    onclick: move |_| timer.with(|t| if !t.always_ring(){bell.always_ring()}),
                    "Ring"
                }
            }
        }
        div { class: "flex flex-row space-x-1 m-1 ",
            div {
            select {
                id: "sequences",
                name: "Sequence",
                class: "select select-success",
                oninput: move |ev| {
                    if let Ok(index) = ev.data.value().parse::<usize>() {
                        timer.with_mut(|t| t.set_sequence(index));
                    }
                },
                option { disabled: true, selected: true, "Select a workout" }
                for (index , sequence) in timer.read().sequences().iter().enumerate() {
                    option {
                        value: index.to_string(),
                        selected: timer.read().sequences().get().map(|s| s.name() == sequence.name()),
                        { sequence.to_string() }
                    }
                }
            }
            if let Some(sequence) = timer.read().sequences().get() {
                if !sequence.is_empty() {
                    ul {
                        id: "sequence",
                        class: "info flex-none p-2 bg-primary-600 rounded-xl bg-sky-900",
                        for (index , item) in sequence.iter().enumerate() {
                            li {
                                class: "text-nowrap",
                                class: if sequence.index() == Some(index) { "text-red-600" } else { "" },
                                span { class: "text-sm", "{item}" }
                            }
                        }
                    }
                }
            }
            }
            div {
                id: "status",
                class: "bg-blue-600 flex w-full items-center justify-center h-screen rounded-xl",
                div { class: "items-center justify-center",
                    div { class: "flex flex-col",
                        if let Some(sequence) = timer.read().sequences().get() {
                            div { class: "items-center justify-center display-grid grid p-12",
                                if let Some(item) = sequence.get() {
                                    div { id: "counter", class: "text-9xl",
                                        { item.left().to_string() }
                                    }
                                }
                                div { id: "status", class: "text-3xl",
                                    "Status: "
                                    { timer.read().status().to_string() }
                                }
                                div { id: "elapsed", class: "text-3xl",
                                    "Elapsed: "
                                    { timer.read().elapsed().to_string() }
                                }
                                div { id: "workout", class: "text-3xl",
                                    "Total Workout: "
                                    { sequence.workout_total().to_string() }
                                }
                                div { id: "rest", class: "text-3xl",
                                    "Total rest: "
                                    { sequence.rest_total().to_string() }
                                }
                                div { id: "left", class: "text-3xl",
                                    "Total left: "
                                    { sequence.left_total().to_string() }
                                }
                                div { id: "total", class: "text-3xl",
                                    "Total: "
                                    { sequence.total().to_string() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
