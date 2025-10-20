use crate::components::{Items, SequenceTimer, Sequences, SequencesJson, Tags, Workouts};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Sequences {},
    #[route("/sequences.json")]
    SequencesJson {},
    #[route("/sequence?:slug")]
    SequenceTimer { slug: String },
    #[route("/tags")]
    Tags {},
    #[route("/items?:slug")]
    Items { slug: String },
    #[route("/timer?:slug")]
    Workouts { slug: String },
}
