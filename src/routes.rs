use crate::item::{all_items, all_tags, TAG_TO_ITEMS};
use crate::mobile::{MobileHome, MobileTimer};
use crate::sequence::all_sequences;
use crate::web::WebHome;
use dioxus::prelude::*;
use itertools::Itertools;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/?:muted&:prepare&:sequence")]
    WebHome {
        muted: bool,
        prepare: u64,
        sequence: String,
    },
    #[route("/sequences")]
    SequencesHome {},
    #[route("/sequences.json")]
    SequencesJson {},
    #[route("/sequence?:name")]
    SequenceHome { name: String },
    #[route("/tags")]
    TagsHome {},
    #[route("/items?:tag")]
    ItemsHome { tag: String },
    #[route("/item?:name")]
    ItemHome { name: String },
    #[route("/mobile")]
    MobileHome {},
    #[route("/timer?:sequence")]
    MobileTimer { sequence: String },
}

#[component]
pub fn SequencesHome() -> Element {
    let sequences = all_sequences();
    rsx! {
        span { {format!("Sequences: {}", sequences.len())} }
        ul { id: "sequences",
            for sequence in sequences {
                li { id: format!("sequence_{}", sequence.slug()),
                    Link {
                        to: Route::SequenceHome {
                            name: sequence.slug(),
                        },
                        {sequence.to_string()}
                    }
                }
            }
        }
    }
}

#[component]
pub fn SequencesJson() -> Element {
    let sequences = all_sequences();
    rsx! {
        pre { {serde_json::to_string_pretty(&sequences).unwrap()} }
    }
}

#[component]
pub fn TagsHome() -> Element {
    let tags = all_tags();
    rsx! {
        span { {format!("Tags: {}", tags.len())} }
        ul { id: "tags",
            for tag in tags {
                li { id: format!("tag_{}", tag.slug()),
                    Link {
                        to: Route::ItemsHome {
                            tag: tag.slug(),
                        },
                        {tag.to_string()}
                    }
                }
            }
        }
    }
}

#[component]
pub fn ItemsHome(tag: String) -> Element {
    let items = if tag.is_empty() {
        all_items()
    } else {
        TAG_TO_ITEMS
            .read()
            .unwrap()
            .iter()
            .filter(|(t, _)| t.slug() == tag)
            .flat_map(|(_, items)| items.iter().cloned().collect_vec())
            .collect_vec()
    };
    rsx! {
        span { {format!("Items: {}", items.len())} }
        ul { id: "items",
            for item in items.iter() {
                li { id: format!("item_{}", item.slug()),
                    Link {
                        to: Route::ItemHome {
                            name: item.slug(),
                        },
                        {item.to_string()}
                    }
                }
            }
        }
    }
}

#[component]
pub fn ItemHome(name: String) -> Element {
    rsx! { "unimplemented" }
}

#[component]
pub fn SequenceHome(name: String) -> Element {
    rsx! { "unimplemented" }
}
