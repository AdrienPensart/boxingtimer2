#![allow(non_snake_case)]
pub mod defaults;
pub mod duration;
pub mod errors;
pub mod exercises;
pub mod global;
pub mod indexedvec;
pub mod item;
pub mod loading;
pub mod mobile;
pub mod mobiletimer;
pub mod routes;
pub mod sequence;
pub mod signal;
pub mod sound;
pub mod status;
pub mod stopwatch;
pub mod tag;
pub mod timer;
pub mod web;
pub mod workout;

use dioxus::logger::tracing::Level;
use dioxus::prelude::*;
use loading::ChildrenOrLoading;

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.png") }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        ChildrenOrLoading { Router::<routes::Route> {} }
    }
}
