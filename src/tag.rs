use derive_more::Display;

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Tag {
    Easy,
    Medium,
    Hard,
    #[display("Warm Up")]
    WarmUp,
    Strengthening,
    Abs,
    Stationary,
    Dynamic,
    Stamina,
    #[display("Full body")]
    FullBody,
    HiiT,
    Prepare,
    Rest,
    Boxing,
}
