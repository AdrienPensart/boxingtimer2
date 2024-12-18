use derive_more::{Deref, DerefMut, Display, From, IntoIterator};

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash, From)]
pub enum Tag {
    #[from]
    Difficulty(Difficulty),
    #[from]
    Body(Body),
    #[from]
    Mouvement(Mouvement),

    // Boxing
    #[display("🥊")]
    Boxing,
    Footwork,
}

// #[repr(transparent)]
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, Deref, DerefMut, IntoIterator, From)]
#[from(forward)]
pub struct Tags(Vec<Tag>);

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Difficulty {
    #[display("🟩")]
    Easy,
    #[display("🟧")]
    Medium,
    #[display("🟥")]
    Hard,
}

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Body {
    Full,
    Abs,
    Pectorals,
    Legs,
}

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mouvement {
    #[display("❤️‍🔥")]
    Cardio,
    #[display("⏳")]
    Rest,
    #[display("🏋")]
    Strengthening,
    #[display("🤸🏼")]
    Stationary,
    #[display("🏃")]
    Dynamic,
}
