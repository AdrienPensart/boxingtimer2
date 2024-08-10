use derive_more::Display;

#[derive(Display, Clone, Copy, Default, Debug, Eq, PartialEq, Hash)]
pub enum Difficulty {
    #[default]
    None,
    Easy,
    Medium,
    Hard,
}
