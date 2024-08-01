#![allow(non_snake_case)]
pub mod bell;
pub mod boxing_timer;
pub mod difficulty;
pub mod duration;
pub mod errors;
pub mod helpers;
pub mod item;
pub mod sequence;
pub mod status;
pub mod stopwatch;
pub mod tag;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
// use gloo::console::log;
use crate::bell::Bell;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/?:muted&:start&:rounds&:fight&:rest&:prepare")]
    BoxingTimer {
        muted: bool,
        start: bool,
        rounds: u64,
        fight: u64,
        rest: u64,
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
fn BoxingTimer(
    muted: bool,
    start: bool,
    rounds: u64,
    fight: u64,
    rest: u64,
    prepare: u64,
) -> Element {
    if rounds == 0 {
        rounds = 12;
    }
    if fight == 0 {
        fight = 180;
    }
    if rest == 0 {
        rest = 60;
    }
    if prepare == 0 {
        prepare = 5;
    }

    let mut bell = use_signal(bell::Bell::new);

    use_effect(move || {
        if muted {
            bell.read().toggle();
        }
    });

    let boxing_timer = match boxing_timer::BoxingTimer::new(prepare, fight, rest, rounds as usize) {
        Ok(boxing_timer) => boxing_timer,
        Err(e) => return rsx! { "{e}" },
    };

    let mut timer = use_signal(|| boxing_timer);

    let _tick = use_resource(move || async move {
        loop {
            gloo::timers::future::TimeoutFuture::new(boxing_timer::DEFAULT_INTERVAL as u32).await;
            timer.write().tick();
        }
    });

    use_effect(move || {
        if start {
            timer.write().toggle()
        }
    });

    rsx! {
        audio {
            id: "bell",
            src: "bell.mp3",
            preload: "auto",
            autoplay: false
        }

        button { onclick: move |_| timer.with_mut(|bt| bt.toggle()),
            {timer.read().next_status().to_string()}
        }
        button { onclick: move |_| timer.with_mut(|bt| bt.reset_beginning()), "Reset beginning" }
        button { onclick: move |_| timer.with_mut(|bt| bt.reset_current()), "Reset current" }
        button { onclick: move |_| timer.with_mut(|bt| bt.goto_previous()), "Previous" }
        button { onclick: move |_| timer.with_mut(|bt| bt.goto_next()), "Next" }

        button { onclick: move |_| bell.with_mut(|bell| bell.toggle()), { bell.read().next_label() } }
        button { onclick: move |_| Bell::ring(), "BELL" }

        div {
            for (index , item) in timer.read().sequence().iter().enumerate() {
                if Some(index) == timer.read().current_item() {
                    div {
                        b { "{item}" }
                    }
                } else {
                    div { "{item}" }
                }
            }
        }
        div {
            "Left: "
            { timer.read().stopwatch().to_string() }
        }
        div {
            "Elapsed: "
            { timer.read().stopwatch().elapsed().to_string() }
        }
        div {
            "Workout: "
            { timer.read().sequence().workout_total().to_string() }
        }
        div {
            "Rest: "
            { timer.read().sequence().waiting_total().to_string() }
        }
        div {
            "Total: "
            { timer.read().sequence().total().to_string() }
        }
        div {
            b {
                "Status: "
                { timer.read().status().to_string() }
            }
        }
    }
}
