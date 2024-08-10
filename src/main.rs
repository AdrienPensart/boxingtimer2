#![allow(non_snake_case)]
pub mod bell;
pub mod difficulty;
pub mod duration;
pub mod errors;
pub mod helpers;
pub mod item;
pub mod sequence;
pub mod status;
pub mod stopwatch;
pub mod tag;
pub mod timer;
use crate::bell::{Bell, BELL_ID};
use crate::duration::DurationExt;
use crate::item::{Prepare, WarmUp};
use crate::sequence::Sequence;
use crate::tag::Tag;
use crate::timer::Timer;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
// use gloo::console::log;
// use manganis::mg;

// const _: &str = mg!(file("assets/tailwind.css"));

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
    let prepare = &std::time::Duration::from_secs(prepare);

    // let test = Sequence::simple(
    //     "Test",
    //     &[
    //         Prepare(prepare),
    //         // 1 minute
    //         WarmUp("test", &std::time::Duration::from_secs(3)),
    //     ],
    // );
    // let test_cycle = Sequence::cycle(
    //     "Test",
    //     &[
    //         Prepare(prepare),
    //         // 1 minute
    //         WarmUp("test", &std::time::Duration::from_secs(3)),
    //     ],
    // );

    let warmup_boxing = Sequence::simple(
        "Warm Up",
        &[
            Prepare(prepare),
            // 1 minute
            WarmUp("Head rotation", &std::time::Duration::from_secs(20)),
            WarmUp("Shoulders rotation", &std::time::Duration::from_secs(20)),
            WarmUp("Arms rotation", &std::time::Duration::from_secs(20)),
            // 1 minute
            WarmUp("Elbows rotation", &std::time::Duration::from_secs(20)),
            WarmUp("Wrists rotation", &std::time::Duration::from_secs(20)),
            WarmUp("Hips rotation", &std::time::Duration::from_secs(20)),
            // 1 minute
            WarmUp("Knees rotation", &std::time::Duration::from_secs(20)),
            WarmUp("Feet rotation", &std::time::Duration::from_secs(20)),
            WarmUp("Heel drop", &std::time::Duration::from_secs(20)),
            // 1 minute
            WarmUp("Leg swings", &std::time::Duration::from_secs(20)),
            WarmUp("Side leg swings", &std::time::Duration::from_secs(20)),
            WarmUp("Single leg touch toes", &std::time::Duration::from_secs(20)),
            // 1 minute
            WarmUp("Butt kicks", &std::time::Duration::from_secs(30)),
            WarmUp("High knees", &std::time::Duration::from_secs(30)),
            // 1 minute
            WarmUp("Jumping jacks", &std::time::Duration::from_secs(30)),
            WarmUp("Mountain climbers", &std::time::Duration::from_secs(30)),
            // 1 minute
            WarmUp("Jump squats", &std::time::Duration::from_secs(30)),
            WarmUp("Push ups", &std::time::Duration::from_secs(30)),
            // 1 minute
            WarmUp("Alternate lunges", &std::time::Duration::from_secs(30)),
            WarmUp("Burpees", &std::time::Duration::from_secs(30)),
        ],
    );

    let shadow_boxing = Sequence::boxing(
        "Shadow boxing: 3x2m + 30s",
        3,
        prepare,
        &std::time::Duration::from_secs(120),
        &std::time::Duration::from_secs(30),
    );

    let heavy_bag = Sequence::boxing(
        "Heavy bag: 3x2m + 30s",
        3,
        prepare,
        &std::time::Duration::from_secs(120),
        &std::time::Duration::from_secs(30),
    );

    let pro_boxing = Sequence::boxing(
        "Pro: 12x3m + 1m",
        12,
        prepare,
        &std::time::Duration::from_secs(180),
        &std::time::Duration::from_secs(60),
    );

    let olympic_boxing = Sequence::boxing(
        "Olympic: 12x2m + 1m",
        12,
        prepare,
        &std::time::Duration::from_secs(120),
        &std::time::Duration::from_secs(60),
    );

    let stamina_boxing = Sequence::stamina(
        "Jab/Cross/Jab-Cross/Jab-Cross-Hook",
        vec!["Jab", "Cross", "Jab/Cross", "Jab/Cross/Hook"],
        prepare,
        &std::time::Duration::from_secs(30),
        &std::time::Duration::from_secs(60),
        4,
    );

    let hiit = Sequence::hiit(
        prepare,
        &std::time::Duration::from_secs(20),
        &std::time::Duration::from_secs(10),
    );

    let jump_role_5mn = Sequence::workout(
        "Jump Rope",
        prepare,
        &std::time::Duration::from_secs(5 * 60),
        &[Tag::Boxing],
    );

    let jump_role_10mn = Sequence::workout(
        "Jump Rope",
        prepare,
        &std::time::Duration::from_secs(10 * 60),
        &[Tag::Boxing],
    );

    let sequences = [
        // test.clone(),
        // test_cycle.clone(),
        warmup_boxing,
        shadow_boxing,
        heavy_bag,
        pro_boxing,
        olympic_boxing,
        stamina_boxing,
        hiit,
        jump_role_5mn,
        jump_role_10mn,
    ];

    let mut bell = use_signal(Bell::default);
    let _mute_bell = use_resource(move || async move {
        if muted {
            bell.write().toggle()
        }
    });

    let mut timer = use_signal(Timer::default);
    let _tick = use_resource(move || async move {
        loop {
            gloo::timers::future::TimeoutFuture::new(timer::DEFAULT_INTERVAL as u32).await;
            timer.write().tick(&bell.read());
        }
    });

    use_effect(move || {
        if start {
            timer.write().toggle();
        }
    });

    rsx! {
        audio {
            id: BELL_ID,
            src: "bell.mp3",
            preload: "auto",
            autoplay: false
        }
        div { class: "contents flex-wrap space-x-2",
            button {
                class: "btn btn-primary rounded-full w-24 m-2",
                onclick: move |_| timer.with_mut(|t| t.toggle()),
                {timer.read().next_status().to_string()}
            }
            button {
                class: "btn btn-primary rounded-full m-2",
                onclick: move |_| timer.with_mut(|t| t.reset_beginning(true)),
                "Restart sequence"
            }
            button {
                class: "btn btn-primary rounded-full m-2",
                onclick: move |_| timer.with_mut(|t| t.reset_current()),
                "Restart current"
            }
            button {
                class: "btn btn-accent rounded-full m-2",
                onclick: move |_| timer.with_mut(|t| t.goto_previous()),
                "Previous"
            }
            button {
                class: "btn btn-accent rounded-full m-2",
                onclick: move |_| timer.with_mut(|t| t.goto_next(true)),
                "Next"
            }
            button {
                class: "btn btn-secondary rounded-full w-24 m-2",
                onclick: move |_| bell.with_mut(|b| b.toggle()),
                { bell.read().next_label() }
            }
            button {
                class: "btn btn-secondary rounded-full m-2",
                onclick: move |_| bell.with(|b| b.always_ring()),
                "BELL"
            }
            select {
                id: "sequences",
                name: "Sequence",
                class: "select select-success",
                oninput: move |ev| {
                    timer
                        .with_mut(|t| {
                            for sequence in sequences.iter() {
                                if sequence.name() == ev.data.value() {
                                    t.set_sequence(sequence.clone());
                                }
                            }
                        })
                },
                option { disabled: true, selected: true, value: true, "Select a workout" }
                for sequence in sequences.iter() {
                    option {
                        value: sequence.name(),
                        selected: timer.read().name() == sequence.name(),
                        { sequence.to_string() }
                    }
                }
            }
        }
        div { class: "flex flex-row space-x-1 m-1 ",
            ul {
                id: "sequence",
                class: "info flex-none p-2 bg-primary-600 rounded-xl bg-sky-900",
                li { class: "text-center",
                    b { { timer.read().name() } }
                }
                for (index , item) in timer.read().iter().enumerate() {
                    li {
                        class: "text-nowrap",
                        class: if timer.read().position() == &index { "text-red-600" } else { "" },
                        span { class: "text-sm", "{item}" }
                    }
                }
            }
            div {
                id: "status",
                class: "bg-blue-600 flex w-full items-center justify-center h-screen rounded-xl",
                div { class: "items-center justify-center",
                    div { class: "flex flex-col",
                        if let Some(stopwatch) = timer.read().stopwatch() {
                            div { id: "counter", class: "text-9xl", { stopwatch.left().to_string() } }
                        }
                        div { class: "items-center justify-center display-grid grid p-12",
                            div { id: "status",
                                "Status: "
                                { timer.read().status().to_string() }
                            }
                            div { id: "elapsed",
                                "Elapsed: "
                                { timer.read().elapsed().to_string() }
                            }
                            div { id: "workout",
                                "Total Workout: "
                                { timer.read().workout_total().to_string() }
                            }
                            div { id: "rest",
                                "Total rest: "
                                { timer.read().waiting_total().to_string() }
                            }
                            div { id: "left",
                                "Total left: "
                                { timer.read().left_total().to_string() }
                            }
                            div { id: "total",
                                "Total: "
                                { timer.read().total().to_string() }
                            }
                        }
                    }
                }
            }
        }
    }
}
