use derive_more::Display;

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tag {
    Prepare,
    #[display(fmt = "Warm Up")]
    WarmUp,
    Strengthening,
    Abs,
    Stationary,
    Dynamic,
    #[display(fmt = "Full body")]
    FullBody,
    Boxing,
}
