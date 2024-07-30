use derive_more::Display;

#[derive(Display, Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum Difficulty {
    #[default]
    Unknown,
    Easy,
    Medium,
    Hard,
}
