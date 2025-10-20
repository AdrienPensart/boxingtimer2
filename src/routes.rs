use crate::mobile::MobileTimer;
use dioxus::prelude::*;
use itertools::Itertools;
use sport::defaults::SEQUENCES;
use sport::item_list::ItemList;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    SequencesHome {},
    #[route("/sequences.json")]
    SequencesJson {},
    #[route("/sequence?:slug")]
    SequenceHome { slug: String },
    #[route("/tags")]
    TagsHome {},
    #[route("/items?:slug")]
    ItemsHome { slug: String },
    #[route("/timer?:slug")]
    MobileTimer { slug: String },
}

#[component]
pub fn SequencesHome() -> Element {
    rsx! {
        ul { id: "sequences",
            for sequence in SEQUENCES.iter() {
                li { id: format!("sequence_{}", sequence.slug()),
                    Link {
                        to: Route::MobileTimer {
                            slug: sequence.slug(),
                        },
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
pub fn TagsHome() -> Element {
    let tags = ItemList::tags();
    rsx! {
        span { {format!("Tags: {}", tags.len())} }
        ul { id: "tags",
            for tag in tags {
                li { id: format!("tag_{}", tag.slug()),
                    Link {
                        to: Route::ItemsHome {
                            slug: tag.slug(),
                        },
                        {tag.to_string()}
                    }
                }
            }
        }
    }
}

#[component]
pub fn ItemsHome(slug: String) -> Element {
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
pub fn SequenceHome(slug: String) -> Element {
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
