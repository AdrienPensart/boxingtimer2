use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Default, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Sound {
    #[default]
    Silent,
    Bell,
    Beep,
}

impl Sound {
    pub fn is_silent(&self) -> bool {
        matches!(self, Self::Silent)
    }
    pub fn is_bell(&self) -> bool {
        matches!(self, Self::Bell)
    }
    pub fn is_beep(&self) -> bool {
        matches!(self, Self::Beep)
    }
}
