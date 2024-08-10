use derive_more::Display;

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Tag {
    Prepare,
    #[display("Warm Up")]
    WarmUp,
    Strengthening,
    Abs,
    Stationary,
    Dynamic,
    #[display("Full body")]
    FullBody,
    Boxing,
    HiiT,
}
