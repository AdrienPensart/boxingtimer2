use crate::audio::Sounds;
use crate::global::Global;
use crate::routes;
use crate::routes::Route;
use dioxus::prelude::*;
use itertools::Itertools;
use sport::defaults::SEQUENCES;
use sport::defaults::{NEXT_ITEM, PREVIOUS_ITEM, RANDOMIZE, RESTART_SEQUENCE, SIGNAL};
use sport::duration::DurationExt;
use sport::item_list::ItemList;
use sport::timer::Timer;

#[component]
pub fn SequenceTimer(slug: String) -> Element {
    let Some(global) = Global::new(false, 10, &slug) else {
        return rsx! { "unknown sequence" };
    };
    if global.timer.read().sequence().is_empty() {
        return rsx! { "empty sequence" };
    }
    let mut global = use_context_provider(|| global);
    let timer = global.timer.read();
    rsx! {
        Sounds {}
        div { id: "timer", class: "grid gap-4 grid-cols-1 text-3xl p-2",
            Controls {}
            div { class: "flex items-center justify-center",
                button {
                    id: "current_workout",
                    title: "Restart workout",
                    onclick: move |_| global.timer.with_mut(Timer::restart_workout),
                    {timer.label()}
                }
            }
            div { class: "flex items-center justify-center",
                span { id: "counter", title: "Time left", {global.timer.read().left().to_string()} }
            }
            if let Some(next_workout) = global.timer.read().sequence().next_workout() {
                div { class: "flex items-center justify-center",
                    span { id: "next_exercise",
                        {"Next: "}
                        {next_workout.item().name()}
                    }
                }
            } else {
                span { class: "flex items-center justify-center", "END" }
            }
            div { class: "flex items-center justify-center",
                Link {
                    id: "exercises_link",
                    title: "See exercises in this sequence",
                    to: routes::Route::Workouts {
                        slug: global.timer.read().sequence().slug(),
                    },
                    {"See exercises"}
                }
            }
            div { class: "flex items-center justify-center",
                Link {
                    id: "home_link",
                    title: "Go to sequence list",
                    to: routes::Route::Sequences {},
                    {"Home"}
                }
            }
        }
    }
}

// #[component]
// pub fn SequenceTimer2(slug: String) -> Element {
//     let Some(global) = Global::new(false, 10, &slug) else {
//         return rsx! { "unknown sequence" };
//     };
//     if global.timer.read().sequence().is_empty() {
//         return rsx! { "empty sequence" };
//     }
//     let mut global = use_context_provider(|| global);
//     let timer = global.timer.read();
//     rsx! {
//         div { class: "container",
//             div { class: "controls",
//                 div { class: "start_stop" }
//                 div { class: "restart_sequence" }
//                 div { class: "previous" }
//                 div { class: "next" }
//                 div { class: "mute" }
//                 div { class: "ring" }
//             }
//             div { class: "timer",
//                 div { class: "current_items",
//                     div { class: "previous_item" }
//                     div { class: "current_item" }
//                     div { class: "next_item" }
//                 }
//                 div { class: "counter" }
//                 div { class: "sequence_items" }
//             }
//             div { class: "links",
//                 div { class: "sequences" }
//                 div { class: "tags" }
//                 div { class: "items" }
//             }
//         }
//     }
// }

#[component]
pub fn Controls() -> Element {
    let mut global = use_context::<Global>();
    rsx! {
        div { id: "controls", class: "flex justify-evenly p-2",
            button {
                id: "toggle_timer",
                class: "rounded-full text-3xl",
                title: global.timer.read().status().next_title(),
                onclick: move |_| global.timer.with_mut(Timer::toggle),
                {global.timer.read().status().next().to_string()}
            }
            button {
                id: "restart_sequence",
                class: "rounded-full text-3xl",
                title: "Restart sequence",
                onclick: move |_| global.timer.with_mut(Timer::restart_sequence),
                {RESTART_SEQUENCE}
            }
            button {
                id: "previous_workout",
                class: "rounded-full text-3xl",
                title: "Previous workout",
                onclick: move |_| global.timer.with_mut(Timer::manual_previous),
                {PREVIOUS_ITEM}
            }
            button {
                id: "next_workout",
                class: "rounded-full text-3xl",
                title: "Next workout",
                onclick: move |_| global.timer.with_mut(Timer::manual_next),
                {NEXT_ITEM}
            }
            if global.timer.read().sequence().shufflable() {
                button {
                    id: "randomize",
                    class: "rounded-full text-3xl",
                    title: "Shuffle sequence",
                    onclick: move |_| global.timer.with_mut(Timer::shuffle),
                    {RANDOMIZE}
                }
            }
            if !global.timer.read().sequence().sound().is_silent() {
                button {
                    id: "toggle_signal",
                    class: "text-3xl",
                    title: global.sound_signal.read().state().next_title(),
                    onclick: move |_| global.sound_signal.with_mut(sport::signal::SoundSignal::toggle),
                    input {
                        r#type: "checkbox",
                        checked: global.sound_signal.read().enabled(),
                        id: "mute_or_unmute",
                    }
                    {global.sound_signal.read().next().to_string()}
                }
                button {
                    id: "emit_signal",
                    class: "text-3xl",
                    title: "Emit signal sound",
                    onclick: move |_| { global.timer.with(|t| { t.ring() }) },
                    {SIGNAL}
                }
            }
        }
    }
}

#[component]
pub fn Sequences() -> Element {
    rsx! {
        ul { id: "sequences",
            for sequence in SEQUENCES.iter() {
                li { id: format!("sequence_{}", sequence.slug()),
                    Link {
                        to: Route::SequenceTimer {
                            slug: sequence.slug(),
                        },
                        title: format!("Start timer for {}", sequence.name()),
                        {sequence.to_string()}
                    }
                }
            }
        }
    }
    // span { {format!("Sequences: {}", SEQUENCES.len())} }
}

#[component]
pub fn SequencesJson() -> Element {
    rsx! {
        pre { {serde_json::to_string_pretty(SEQUENCES.as_slice()).unwrap()} }
    }
}

#[component]
pub fn Tags() -> Element {
    let tags = ItemList::tags();
    rsx! {
        span { {format!("Tags: {}", tags.len())} }
        ul { id: "tags",
            for tag in tags {
                li { id: format!("tag_{}", tag.slug()),
                    Link { to: Route::Items { slug: tag.slug() }, {tag.to_string()} }
                }
            }
        }
    }
}

#[component]
pub fn Items(slug: String) -> Element {
    let items = if slug.is_empty() {
        ItemList::items()
    } else {
        ItemList::tag_to_items()
            .iter()
            .filter(|(t, _)| t.slug() == slug)
            .flat_map(|(_, items)| items.iter().cloned().collect_vec())
            .collect_vec()
    };
    rsx! {
        span { {format!("Items: {}", items.len())} }
        ul { id: "items",
            for item in items.iter() {
                li { id: format!("item_{}", item.slug()), {item.to_string()} }
            }
        }
    }
}

#[component]
pub fn Workouts(slug: String) -> Element {
    let Some(sequence) = SEQUENCES.iter().find(|s| s.slug() == slug) else {
        return rsx! { "unknown sequence" };
    };

    rsx! {
        ul { id: "workouts",
            for item in sequence.unique_items().iter() {
                li { id: format!("item_{}", item.slug()), {item.name()} }
            }
        }
    }
}
