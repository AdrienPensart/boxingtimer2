#![allow(non_snake_case)]
pub mod beep;
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
use crate::duration::{MINUTE, SECOND};
use crate::item::{Easy, Medium, Workout};
use crate::sequence::{Sequence, ROUNDS};
use crate::signal::{Signal, State};
use crate::sound::Sound;
use crate::tag::Tag;
use dioxus::prelude::*;
// use dioxus_logger::tracing::error;
use dioxus_logger::tracing::Level;
use manganis::mg;
use std::{cell::RefCell, rc::Rc};

const _: &str = mg!(file("assets/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/?:muted&:prepare")]
    BoxingTimer { muted: bool, prepare: u64 },
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
fn BoxingTimer(muted: bool, prepare: u64) -> Element {
    let state = if muted {
        State::Disabled
    } else {
        State::Enabled
    };
    let state = Rc::new(RefCell::new(state));

    let silent = Signal::new(Sound::Silent, state.clone());
    let bell = Signal::new(Sound::Bell, state.clone());
    let beep = Signal::new(Sound::Beep, state.clone());

    let preparation = std::time::Duration::from_secs(prepare);

    let warmup = Sequence::simple(
        "Warm-up : 🌡",
        &[
            // 1 minute
            Easy("Head rotation", 20 * SECOND),
            Easy("Shoulders rotation", 20 * SECOND),
            Easy("Arms rotation", 20 * SECOND),
            // 1 minute
            Easy("Elbows rotation", 20 * SECOND),
            Easy("Wrists rotation", 20 * SECOND),
            Easy("Hips rotation", 20 * SECOND),
            // 1 minute
            Easy("Knees rotation", 20 * SECOND),
            Easy("Feet rotation", 20 * SECOND),
            Easy("Heel raises", 20 * SECOND),
            // 1 minute
            Easy("Leg swings", 20 * SECOND),
            Easy("Side leg swings", 20 * SECOND),
            Easy("Single leg touch toes", 20 * SECOND),
            // 1 minute
            Easy("Butt kicks", 30 * SECOND),
            Easy("High knees", 30 * SECOND),
            // 1 minute
            Easy("Jumping jacks", 30 * SECOND),
            Easy("Mountain climbers", 30 * SECOND),
            // 1 minute
            Medium("Jump squats", 30 * SECOND),
            Medium("Push ups", 30 * SECOND),
            // 1 minute
            Easy("Speed steps", 30 * SECOND),
            Easy("Left/right jumps", 30 * SECOND),
            // 1 minute
            Medium("Alternate lunges", 30 * SECOND),
            Medium("Burpees", 30 * SECOND),
        ],
        &silent,
    );

    let cardio_warmup = Sequence::simple(
        "Cardio Warm-up : ❤️‍🔥",
        &[
            // 1 minute
            Easy("Jumping jacks", 15 * SECOND),
            Easy("High knees", 30 * SECOND),
            Easy("Jumping jacks", 15 * SECOND),
            Easy("Butt kicks", 30 * SECOND),
            // 1 minute
            Easy("Jumping jacks", 15 * SECOND),
            Medium("Push ups", 15 * SECOND),
            Easy("Jumping jacks", 15 * SECOND),
            Easy("Mountain climbers", 15 * SECOND),
            // 1 minute
            Easy("Jumping jacks", 15 * SECOND),
            Medium("Jump squats", 15 * SECOND),
            Easy("Jumping jacks", 15 * SECOND),
            Medium("Alternate lunges", 15 * SECOND),
            // 1 minute
            Easy("Jumping jacks", 15 * SECOND),
            Easy("Speed steps", 30 * SECOND),
            Easy("Jumping jacks", 15 * SECOND),
            Medium("Burpees", 30 * SECOND),
            // 1 minute
            Easy("Jumping jacks", 15 * SECOND),
            Easy("Crunches", 15 * SECOND),
            Easy("Jumping jacks", 15 * SECOND),
            Easy("Plank", 15 * SECOND),
        ],
        &beep,
    );

    let boxing_3x2m_30s = Sequence::rounds(
        "3x2m (30s rest)",
        3 * ROUNDS,
        Workout("Boxing Round", 2 * MINUTE, &[Tag::Boxing, Tag::Medium]),
        30 * SECOND,
        &bell,
    );

    let boxing_3x3m_1m = Sequence::rounds(
        "3x3m (60s rest)",
        3 * ROUNDS,
        Workout("Boxing Round", 3 * MINUTE, &[Tag::Boxing, Tag::Medium]),
        60 * SECOND,
        &bell,
    );

    let boxing_6x2m_30s = Sequence::rounds(
        "6x2m (30s rest)",
        6 * ROUNDS,
        Workout("Boxing Round", 2 * MINUTE, &[Tag::Boxing, Tag::Medium]),
        30 * SECOND,
        &bell,
    );

    let boxing_6x3m_1m = Sequence::rounds(
        "6x3m (60s rest)",
        6 * ROUNDS,
        Workout("Boxing Round", 3 * MINUTE, &[Tag::Boxing, Tag::Medium]),
        60 * SECOND,
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
        30 * SECOND,
        4 * ROUNDS,
        1 * MINUTE,
        &[Tag::Boxing, Tag::Medium],
        &bell,
    );

    let stamina_jab_jab_cross_cross = Sequence::repeat(
        "1 | 1-1 | 1-1-2 | 1-1-2-2 (60s rest)",
        vec![
            "Jab (1)",
            "Double Jab (1-1)",
            "Double Jab | Cross (1-1-2)",
            "Double Jab | Cross | Cross",
        ],
        30 * SECOND,
        4 * ROUNDS,
        1 * MINUTE,
        &[Tag::Boxing, Tag::Medium],
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
        30 * SECOND,
        4 * ROUNDS,
        1 * MINUTE,
        &[Tag::Boxing, Tag::Medium],
        &bell,
    );

    let random_hiit = Sequence::random(
        "Random training 🎲",
        &[
            Workout("Jumping Jacks", 30 * SECOND, &[Tag::Dynamic]),
            Workout("Pull-ups", 30 * SECOND, &[Tag::Dynamic]),
            Workout("Plank", 30 * SECOND, &[Tag::Stationary]),
            Workout("Jump Squats", 30 * SECOND, &[Tag::Dynamic]),
            Workout("Burpees", 30 * SECOND, &[Tag::Dynamic]),
            Workout("Push-ups", 30 * SECOND, &[Tag::Dynamic]),
            Workout("Alternate Lunges", 30 * SECOND, &[Tag::Dynamic]),
        ],
        30 * SECOND,
        &beep,
    );

    let hiit = Sequence::infinite(
        Workout("Workout!", 20 * SECOND, &[Tag::Hard]),
        10 * SECOND,
        &beep,
    );
    let _5mn = Sequence::workout("5mn", Workout("Workout", 5 * MINUTE, &[Tag::Easy]), &bell);
    let _10mn = Sequence::workout(
        "10mn",
        Workout("Workout", 10 * MINUTE, &[Tag::Medium]),
        &bell,
    );
    let _15mn = Sequence::workout("15mn", Workout("Workout", 15 * MINUTE, &[Tag::Hard]), &bell);

    let mut state_signal = use_signal(|| state.clone());

    let mut timer = use_signal(|| {
        timer::Timer::new(
            preparation,
            &[
                warmup,
                cardio_warmup,
                boxing_3x2m_30s,
                boxing_3x3m_1m,
                boxing_6x2m_30s,
                boxing_6x3m_1m,
                stamina_jab_cross_hook,
                stamina_jab_jab_cross_cross,
                stamina_jab_cross_hook_cross,
                hiit,
                random_hiit,
                _5mn,
                _10mn,
                _15mn,
            ],
        )
    });
    let _tick = use_resource(move || async move {
        loop {
            gloo::timers::future::TimeoutFuture::new(timer::DEFAULT_INTERVAL).await;
            if timer.write().tick() {
                gloo::timers::future::TimeoutFuture::new(timer::DEFAULT_INTERVAL).await;
            }
        }
    });

    rsx! {
        div { id: "controls", class: "flex flex-wrap space-x-2",
            div { id: "timer_controls",
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
            div { id: "sequence_controls",
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
                if timer.read().sequences().get().is_some() {
                    button {
                        class: "btn btn-accent rounded-full m-2",
                        onclick: move |_| timer.with_mut(|t| t.shuffle()),
                        "Shuffle"
                    }
                }
            }
            div { id: "sounds_controls",
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
                    id: "toggle_signal",
                    class: "btn btn-secondary rounded-full w-24 m-2",
                    onclick: move |_| state_signal.with_mut(|s| s.borrow_mut().toggle()),
                    { state_signal.read().borrow().next_label() }
                }
                button {
                    id: "ring",
                    class: "btn btn-secondary rounded-full m-2",
                    onclick: move |_| {
                        timer
                            .with(|t| {
                                if !t.always_ring() {
                                    bell.always_ring()
                                }
                            })
                    },
                    "Ring"
                }
            }
        }
        div { class: "flex flex-row space-x-1 m-1 ",
            div { class: "space-y-1.5",
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
                            class: "info flex-none p-2 rounded-xl bg-sky-900",
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
                div {
                    id: "informations",
                    class: "rounded-xl bg-sky-900 text-2xl p-2",
                    div { id: "status",
                        "Status: "
                        { timer.read().status().to_string() }
                    }
                    div { id: "elapsed",
                        "Elapsed: "
                        { timer.read().elapsed().to_string() }
                    }
                    if let Some(sequence) = timer.read().sequences().get() {
                        // div { class: "items-center justify-center display-grid grid p-12",
                        div { id: "workout",
                            "Workout: "
                            { sequence.workout_total().to_string() }
                        }
                        div { id: "rest",
                            "Rest: "
                            { sequence.rest_total().to_string() }
                        }
                        div { id: "left",
                            "Left: "
                            { sequence.left_total().to_string() }
                        }
                        div { id: "total",
                            "Total: "
                            { sequence.total().to_string() }
                        }
                    }
                }
            }
            div {
                id: "timer",
                class: "bg-blue-600 flex w-full items-center justify-center h-screen rounded-xl",
                div { class: "flex flex-col items-center justify-center",
                    div { id: "item", class: "text-9xl", { timer.read().label() } }
                    div { id: "counter", class: "text-9xl", { timer.read().left().to_string() } }
                }
            }
        }
    }
}
