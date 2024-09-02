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

    let warmup = Sequence::simple()
        .name("Warm-up : 🌡")
        .items(&[
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
        ])
        .signal(&silent)
        .call();

    let cardio_warmup = Sequence::simple()
        .name("Cardio Warm-up : ❤️‍🔥")
        .items(&[
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
        ])
        .signal(&beep)
        .call();

    let workout_10m = Sequence::rounds()
        .name("1m/10x Workout")
        .rounds(10)
        .workout(Workout("Workout!", 1 * MINUTE, &[]))
        .rest(1 * MINUTE)
        .signal(&beep)
        .call();

    let boxing_3x2m_30s = Sequence::rounds()
        .name("3x2m (30s rest)")
        .rounds(3 * ROUNDS)
        .workout(Workout(
            "Boxing Round",
            2 * MINUTE,
            &[Tag::Boxing, Tag::Medium],
        ))
        .rest(30 * SECOND)
        .signal(&bell)
        .call();

    let boxing_3x3m_1m = Sequence::rounds()
        .name("3x3m (60s rest)")
        .rounds(3 * ROUNDS)
        .workout(Workout(
            "Boxing Round",
            3 * MINUTE,
            &[Tag::Boxing, Tag::Medium],
        ))
        .rest(60 * SECOND)
        .signal(&bell)
        .call();

    let boxing_6x2m_30s = Sequence::rounds()
        .name("6x2m (30s rest)")
        .rounds(6 * ROUNDS)
        .workout(Workout(
            "Boxing Round",
            2 * MINUTE,
            &[Tag::Boxing, Tag::Medium],
        ))
        .rest(30 * SECOND)
        .signal(&bell)
        .call();

    let boxing_6x3m_1m = Sequence::rounds()
        .name("6x3m (60s rest)")
        .rounds(6 * ROUNDS)
        .workout(Workout(
            "Boxing Round",
            3 * MINUTE,
            &[Tag::Boxing, Tag::Medium],
        ))
        .rest(60 * SECOND)
        .signal(&bell)
        .call();

    let stamina_jab_cross_hook = Sequence::repeat()
        .name("1 | 2 | 1-2 | 1-2-3 (60s rest)")
        .names(vec![
            "Jab (1)",
            "Cross (2)",
            "Jab | Cross (1-2)",
            "Jab | Cross | Hook (1-2-3)",
        ])
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(1 * MINUTE)
        .tags(&[Tag::Boxing, Tag::Medium])
        .signal(&bell)
        .call();

    let stamina_jab_jab_cross_cross = Sequence::repeat()
        .name("1 | 1-1 | 1-1-2 | 1-1-2-2 (60s rest)")
        .names(vec![
            "Jab (1)",
            "Double Jab (1-1)",
            "Double Jab | Cross (1-1-2)",
            "Double Jab | Cross | Cross",
        ])
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(1 * MINUTE)
        .tags(&[Tag::Boxing, Tag::Medium])
        .signal(&bell)
        .call();

    let stamina_jab_cross_hook_cross = Sequence::repeat()
        .name("1 | 1-2 | 1-2-3 | 1-2-3-2 (60s rest)")
        .names(vec![
            "Jab (1)",
            "Jab | Cross (1-2)",
            "Jab | Cross | Hook (1-2-3)",
            "Jab | Cross | Hook | Cross (1-2-3-2)",
        ])
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(1 * MINUTE)
        .tags(&[Tag::Boxing, Tag::Medium])
        .signal(&bell)
        .call();

    let random_hiit = Sequence::random()
        .name("Random training 🎲")
        .items(&[
            Workout("Jumping Jacks", 30 * SECOND, &[Tag::Dynamic]),
            Workout("Pull-ups", 30 * SECOND, &[Tag::Dynamic]),
            Workout("Plank", 30 * SECOND, &[Tag::Stationary]),
            Workout("Jump Squats", 30 * SECOND, &[Tag::Dynamic]),
            Workout("Burpees", 30 * SECOND, &[Tag::Dynamic]),
            Workout("Push-ups", 30 * SECOND, &[Tag::Dynamic]),
            Workout("Alternate Lunges", 30 * SECOND, &[Tag::Dynamic]),
        ])
        .rest(30 * SECOND)
        .signal(&beep)
        .call();

    let hiit = Sequence::rounds()
        .name("HiiT 20s/10s (4x)")
        .rounds(4 * ROUNDS)
        .workout(Workout("Workout!", 20 * SECOND, &[Tag::Hard]))
        .rest(10 * SECOND)
        .signal(&beep)
        .call();

    let _5mn = Sequence::workout()
        .name("5mn")
        .workout(Workout("Workout", 5 * MINUTE, &[Tag::Easy]))
        .signal(&bell)
        .call();

    let _10mn = Sequence::workout()
        .name("10mn")
        .workout(Workout("Workout", 10 * MINUTE, &[Tag::Medium]))
        .signal(&bell)
        .call();

    let _15mn = Sequence::workout()
        .name("15mn")
        .workout(Workout("Workout", 15 * MINUTE, &[Tag::Hard]))
        .signal(&bell)
        .call();

    let mut state_signal = use_signal(|| state.clone());

    let mut timer = use_signal(|| {
        timer::Timer::new(
            preparation,
            &[
                warmup,
                cardio_warmup,
                workout_10m,
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
        div { id: "controls", class: "flex flex-wrap space-x-3",
            div { id: "timer_controls", class: "space-x-1.5",
                button {
                    class: "rounded-full text-3xl",
                    onclick: move |_| timer.with_mut(|t| t.toggle()),
                    {timer.read().next_status().to_string()}
                }
                button {
                    class: "rounded-full text-3xl",
                    onclick: move |_| timer.with_mut(|t| t.restart_sequence()),
                    "🗘"
                }
            }
            div { id: "sequence_controls", class: "space-x-1.5",
                button {
                    class: "rounded-full text-3xl",
                    onclick: move |_| timer.with_mut(|t| t.manual_previous()),
                    "⏪"
                }
                button {
                    class: "rounded-full text-3xl",
                    onclick: move |_| timer.with_mut(|t| t.manual_next()),
                    "⏩"
                }
                if timer.read().sequences().get().is_some() {
                    button {
                        class: "rounded-full text-3xl",
                        onclick: move |_| timer.with_mut(|t| t.shuffle()),
                        "🎲"
                    }
                }
            }
            div { id: "sounds_controls", class: "space-x-1.5",
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
                    class: "text-3xl",
                    onclick: move |_| state_signal.with_mut(|s| s.borrow_mut().toggle()),
                    { state_signal.read().borrow().next().to_string() }
                }
                button {
                    id: "ring",
                    class: "text-3xl",
                    onclick: move |_| {
                        timer
                            .with(|t| {
                                if !t.always_ring() {
                                    bell.always_ring()
                                }
                            })
                    },
                    "🛎"
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
                    // div { id: "status",
                    //     "Status: "
                    //     { timer.read().status().to_string() }
                    // }
                    div { id: "elapsed",
                        "Elapsed: "
                        { timer.read().elapsed().to_string() }
                    }
                    if let Some(sequence) = timer.read().sequences().get() {
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
                    button {
                        class: "text-3xl",
                        onclick: move |_| timer.with_mut(|t| t.restart_item()),
                        "🗘"
                    }
                }
            }
        }
    }
}
