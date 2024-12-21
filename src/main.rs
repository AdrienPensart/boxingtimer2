#![allow(non_snake_case)]
pub mod defaults;
pub mod duration;
pub mod errors;
pub mod exercises;
pub mod global;
pub mod indexedvec;
pub mod item;
pub mod loading;
pub mod mobileglobal;
pub mod mobiletimer;
pub mod sequence;
pub mod signal;
pub mod sound;
pub mod status;
pub mod stopwatch;
pub mod tag;
pub mod timer;
pub mod workout;
use crate::duration::DurationExt;
use crate::global::Global;
use crate::signal::Signal;
use dioxus::prelude::*;
// use dioxus_free_icons::icons::io_icons::IoTimeSharp;
// use dioxus_free_icons::Icon;
use dioxus_logger::tracing::Level;
use loading::ChildrenOrLoading;
use mobileglobal::MobileGlobal;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/?:muted&:prepare&:sequence")]
    Home {
        muted: bool,
        prepare: u64,
        sequence: String,
    },
    #[route("/mobile")]
    MobileHome {},
    #[route("/timer?:sequence")]
    MobileTimer { sequence: String },
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.png") }
        // Icon {
        //     width: 30,
        //     height: 30,
        //     fill: "black",
        //     icon: IoTimeSharp,
        // }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        ChildrenOrLoading { Router::<Route> {} }
    }
}

#[component]
fn MobileHome() -> Element {
    let global = Global::new(false, 10, None);
    rsx! {
        ul {
            for sequence in global.sequences.iter() {
                li {
                    Link {
                        to: Route::MobileTimer {
                            sequence: sequence.slug(),
                        },
                        {sequence.to_string()}
                    }
                }
            }
        }
    }
}

#[component]
fn Sounds(bell: Signal, beep: Signal) -> Element {
    rsx! {
        div { id: "sounds",
            audio {
                id: bell.to_string(),
                src: asset!("/assets/Bell.mp3"),
                preload: "auto",
                autoplay: false,
            }
            audio {
                id: beep.to_string(),
                src: asset!("/assets/Beep.mp3"),
                preload: "auto",
                autoplay: false,
            }
        }
    }
}

#[component]
fn MobileTimer(sequence: String) -> Element {
    let Some(global) = MobileGlobal::new(false, 10, sequence) else {
        return rsx! { "Unknown sequence" };
    };
    if global.timer.read().sequence().is_empty() {
        return rsx! { "Empty sequence" };
    }
    let mut global = use_context_provider(|| global);

    rsx! {
        Sounds { bell: global.bell, beep: global.beep }
        MobileControls {}
        div { id: "timer", class: "flex",
            label { id: "item", {global.timer.read().label()} }
            label { id: "counter", {global.timer.read().left().to_string()} }
            button { onclick: move |_| global.timer.with_mut(|t| t.restart_item()), "♻" }
        }
        if !global.timer.read().sequence().is_empty() {
            ul {
                id: "sequence",
                class: "info flex-none p-2 rounded-xl bg-sky-900",
                for (index , item) in global.timer.read().sequence().iter().enumerate() {
                    li {
                        class: "text-nowrap",
                        class: if global.timer.read().sequence().index() == Some(index) { "text-red-600" } else { "" },
                        span { class: "text-sm", "{item}" }
                    }
                }
            }
        }
    }
}

#[component]
fn MobileControls() -> Element {
    let mut global = use_context::<MobileGlobal>();
    rsx! {
        div { id: "controls", class: "flex justify-evenly",
            button {
                class: "rounded-full text-3xl",
                onclick: move |_| global.timer.with_mut(|t| t.toggle()),
                {global.timer.read().next_status().to_string()}
            }
            button {
                class: "rounded-full text-3xl",
                onclick: move |_| global.timer.with_mut(|t| t.restart_sequence()),
                "♻"
            }
            button {
                class: "rounded-full text-3xl",
                onclick: move |_| global.timer.with_mut(|t| t.manual_previous()),
                "⏪"
            }
            button {
                class: "rounded-full text-3xl",
                onclick: move |_| global.timer.with_mut(|t| t.manual_next()),
                "⏩"
            }
            if global.timer.read().sequence().shufflable() {
                button {
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.shuffle()),
                    "🎲"
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
                    "🛎"
                }
            }
        }
    }
}

#[component]
fn Controls(global: Global) -> Element {
    rsx! {
        if let Some(sequence) = global.timer.read().sequences().get() {
            div { id: "controls", class: "flex justify-evenly",
                button {
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.toggle()),
                    {global.timer.read().next_status().to_string()}
                }
                button {
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.restart_sequence()),
                    "♻"
                }
                button {
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.manual_previous()),
                    "⏪"
                }
                button {
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.manual_next()),
                    "⏩"
                }
                if sequence.shufflable() {
                    button {
                        class: "rounded-full text-3xl",
                        onclick: move |_| global.timer.with_mut(|t| t.shuffle()),
                        "🎲"
                    }
                }
                if !sequence.signal().sound().is_silent() {
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
                        "🛎"
                    }
                }
            }
        }
    }
}

#[component]
fn Home(muted: bool, prepare: u64, sequence: String) -> Element {
    let mut global = Global::new(muted, prepare, Some(sequence));

    rsx! {
        div { class: "flex flex-row space-x-1 m-1 ",
            div { id: "left_panel", class: "space-y-1.5",
                Controls { global: global.clone() }
                select {
                    id: "sequences",
                    name: "Sequence",
                    class: "select select-success",
                    oninput: move |ev| {
                        if let Ok(index) = ev.data.value().parse::<usize>() {
                            global
                                .timer
                                .with_mut(|t| {
                                    if let Some(slug) = t.set_sequence(index) {
                                        let nav = navigator();
                                        nav.push(Route::Home {
                                            muted,
                                            prepare,
                                            sequence: slug,
                                        });
                                    }
                                });
                        }
                    },
                    option { disabled: true, selected: true, "Select a workout" }
                    for (index , sequence) in global.timer.read().sequences().iter().enumerate() {
                        option {
                            value: index.to_string(),
                            selected: global.timer.read().sequences().get().map(|s| s.name() == sequence.name()),
                            {sequence.to_string()}
                        }
                    }
                }
                if let Some(sequence) = global.timer.read().sequences().get() {
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
                    div { id: "elapsed",
                        "Elapsed: "
                        {global.timer.read().elapsed().to_string()}
                    }
                    if let Some(sequence) = global.timer.read().sequences().get() {
                        div { id: "workout",
                            "Workout: "
                            {sequence.workout_total().to_string()}
                        }
                        if !sequence.rest_total().is_zero() {
                            div { id: "rest",
                                "Rest: "
                                {sequence.rest_total().to_string()}
                            }
                        }
                        div { id: "left",
                            "Left: "
                            {sequence.left_total().to_string()}
                        }
                        if !sequence.rest_total().is_zero() {
                            div { id: "total",
                                "Total: "
                                {sequence.total().to_string()}
                            }
                        }
                    }
                }
                Sounds { bell: global.bell, beep: global.beep }
            }
            div {
                id: "timer",
                class: "bg-blue-600 flex w-full items-center justify-center h-screen rounded-xl",
                div { class: "flex flex-col items-center justify-center",
                    div { id: "item", class: "text-9xl", {global.timer.read().label()} }
                    div { id: "counter", class: "text-9xl",
                        {global.timer.read().left().to_string()}
                    }
                    button {
                        class: "text-3xl",
                        onclick: move |_| global.timer.with_mut(|t| t.restart_item()),
                        "♻"
                    }
                }
            }
        }
    }
}
