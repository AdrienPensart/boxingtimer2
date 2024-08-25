use derive_more::Display;

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Tag {
    // Difficulty
    #[display("🟩")]
    Easy,
    #[display("🟧")]
    Medium,
    #[display("🟥")]
    Hard,
    #[display("❤️‍🔥")]
    Cardio,
    #[display("⏳")]
    Rest,
    #[display("🏋")]
    Strengthening,

    // Boxing
    #[display("🥊")]
    Boxing,
    Footwork,

    // Body parts
    Abs,
    Pectorals,

    // Mouvement?
    #[display("🤸🏼")]
    Stationary,
    #[display("🏃")]
    Dynamic,
}
