#![allow(non_snake_case)]
pub mod audio;
pub mod errors;
pub mod loading;
pub mod mobile;
pub mod mobiletimer;
pub mod routes;
pub mod signal;
pub mod status;
pub mod timer;
pub mod web;

use dioxus::logger::tracing::Level;
use dioxus::prelude::*;
use loading::ChildrenOrLoading;

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();
    // use item::Item;
    // let items_str = include_str!("../assets/items.json");
    // let items: Vec<Item> = serde_json::from_str(items_str).expect("failed to parse items");
    // items.into_iter().for_each(|item| {
    //     item.register();
    // });

    // use sequence::Sequence;
    // let sequences_str = include_str!("../assets/sequences.json");
    // let sequences: Vec<Sequence> =
    //     serde_json::from_str(sequences_str).expect("failed to parse sequences");
    // sequences.into_iter().for_each(|sequence| {
    //     sequence.register();
    // });

    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.png") }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        ChildrenOrLoading { Router::<routes::Route> {} }
    }
}
