use crate::mobile::{MobileHome, MobileTimer};
use crate::web::WebHome;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/?:muted&:prepare&:sequence")]
    WebHome {
        muted: bool,
        prepare: u64,
        sequence: String,
    },
    #[route("/mobile")]
    MobileHome {},
    #[route("/timer?:sequence")]
    MobileTimer { sequence: String },
}
