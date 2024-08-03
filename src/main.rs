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
use crate::duration::Duration;
use crate::sequence::Sequence;
use crate::timer::Timer;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
// use gloo::console::log;
// use manganis::mg;

// const _: &str = mg!(file("assets/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/?:muted&:start&:prepare&:boxing_rounds&:boxing_fight&:boxing_rest&:hiit_workout&:hiit_rest&:sequence")]
    BoxingTimer {
        muted: bool,
        start: bool,
        prepare: u64,
        boxing_rounds: u64,
        boxing_fight: u64,
        boxing_rest: u64,
        hiit_workout: u64,
        hiit_rest: u64,
        sequence: String,
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
fn BoxingTimer(
    muted: bool,
    start: bool,
    prepare: u64,
    boxing_rounds: u64,
    boxing_fight: u64,
    boxing_rest: u64,
    hiit_workout: u64,
    hiit_rest: u64,
    sequence: String,
) -> Element {
    if boxing_rounds == 0 {
        boxing_rounds = 12;
    }
    if boxing_fight == 0 {
        boxing_fight = 180;
    }
    if boxing_rest == 0 {
        boxing_rest = 60;
    }
    if prepare == 0 {
        prepare = 5;
    }
    if hiit_workout == 0 {
        hiit_workout = 20
    }
    if hiit_rest == 0 {
        hiit_rest = 10
    }

    let boxing = match Sequence::boxing_sequence(
        Duration::from_secs(prepare),
        boxing_rounds as usize,
        Duration::from_secs(boxing_fight),
        Duration::from_secs(boxing_rest),
    ) {
        Ok(boxing) => boxing,
        Err(error) => return rsx! { "{error}" },
    };

    let hiit = match Sequence::hiit_sequence(
        Duration::from_secs(hiit_workout),
        Duration::from_secs(hiit_rest),
    ) {
        Ok(hiit) => hiit,
        Err(error) => return rsx! { "{error}" },
    };

    if sequence.is_empty() {
        sequence = hiit.name().clone()
    }

    let sequences = std::collections::BTreeMap::from([
        (boxing.name().clone(), boxing.clone()),
        (hiit.name().clone(), hiit.clone()),
    ]);

    let mut bell = use_signal(Bell::default);
    let _mute_bell = use_resource(move || async move {
        if muted {
            bell.write().toggle()
        }
    });

    let mut timer = use_signal(|| Timer::new(sequences[&sequence].clone()));
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
                onclick: move |_| timer.with_mut(|t| t.reset_beginning()),
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
                onclick: move |_| timer.with_mut(|t| t.goto_next()),
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
                oninput: move |ev| timer.with_mut(|t| t.set_sequence(&sequences[&ev.data.value()])),
                for (name , _) in sequences.iter() {
                    option {
                        value: "{name}",
                        selected: timer.read().sequence().name() == name,
                        "{name}"
                    }
                }
            }
        }
        div { class: "flex flex-row space-x-1 m-1",
            ul {
                id: "sequence",
                class: "info flex-none p-2 bg-primary-600 rounded-xl bg-sky-900",
                li { class: "text-center",
                    b { "{ timer.read().sequence().name() }" }
                }
                for (index , item) in timer.read().sequence().iter().enumerate() {
                    li {
                        class: "text-nowrap",
                        class: if timer.read().current_item() == Some(index) { "text-red-600" },
                        span { "{item}" }
                    }
                }
            }
            div {
                id: "status",
                class: "bg-blue-600 flex w-full items-center justify-center h-screen rounded-xl",
                div { class: "items-center justify-center",
                    div { class: "flex flex-col",
                        div { id: "counter", class: "text-9xl",
                            { timer.read().stopwatch().to_string() }
                        }
                        div { class: "items-center justify-center display-grid grid p-12",
                            div { id: "elapsed",
                                "Elapsed: "
                                { timer.read().stopwatch().elapsed().to_string() }
                            }
                            div { id: "workout",
                                "Workout: "
                                { timer.read().sequence().workout_total().to_string() }
                            }
                            div { id: "rest",
                                "Rest: "
                                { timer.read().sequence().waiting_total().to_string() }
                            }
                            div { id: "total",
                                "Total: "
                                { timer.read().sequence().total().to_string() }
                            }
                        }
                    }
                }
            }
        }
    }
}
