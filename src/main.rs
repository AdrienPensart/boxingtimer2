#![allow(non_snake_case)]
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
use lenient_bool::LenientBool;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
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
fn BoxingTimer() -> Element {
    let muted = helpers::get_param_or::<LenientBool>("muted", LenientBool(false));
    if muted.into() {
        helpers::BoxingBell::mute()
    }

    let boxing_timer = match boxing_timer::BoxingTimer::new() {
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
        button { onclick: move |_| timer.with_mut(|bt| bt.goto_next()), "Next" }

        button { onclick: move |_| helpers::BoxingBell::toggle(),
            if helpers::BoxingBell::muted() {
                "Unmute"
            } else {
                "Mute"
            }
        }
        button { onclick: move |_| helpers::BoxingBell::play(), "BELL" }

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

#[component]
fn Home() -> Element {
    rsx! {
        BoxingTimer {}
    }
}
