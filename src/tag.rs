use derive_more::{Deref, DerefMut, Display, From, IntoIterator};
use serde::{Deserialize, Serialize};
use slug::slugify;

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash, From, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tag {
    Rest,
    Drink,
    HiiT,
    WarmUp,
    Boxing,
    #[from]
    #[serde(untagged)]
    Body(Body),
    #[from]
    #[serde(untagged)]
    Mouvement(Mouvement),
    #[from]
    #[serde(untagged)]
    Equipment(Equipment),
    #[from]
    #[serde(untagged)]
    Plank(Plank),
    #[from]
    #[serde(untagged)]
    Squat(Squat),
}

impl Tag {
    pub fn icon(&self) -> Option<char> {
        match self {
            Self::Drink => Some('🍹'),
            Self::Rest => Some('💤'),
            Self::Mouvement(mouvement) => Some(mouvement.icon()),
            Self::Plank(_) => Some('🚪'),
            Self::HiiT => Some('🧨'),
            Self::WarmUp => Some('🔥'),
            Self::Boxing => Some('🥊'),
            _ => None,
        }
    }
    pub fn slug(&self) -> String {
        slugify(self.to_string())
    }
}

#[derive(
    Default,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Deref,
    DerefMut,
    IntoIterator,
    From,
    Serialize,
    Deserialize,
)]
#[from(forward)]
// #[serde(transparent)]
pub struct Tags(Vec<Tag>);

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Elite,
}

impl Difficulty {
    pub fn icon(&self) -> char {
        match self {
            Self::Easy => '🟩',
            Self::Medium => '🟨',
            Self::Hard => '🟧',
            Self::Elite => '🟥',
        }
    }
}

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Plank {
    Plank,
    Side,
    Commando,
    Jack,
    ShoulderTap,
    Spiderman,
}

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Squat {
    Squat,
    Jump,
    Sumo,
    SingleLeg,
    Bulgarian,
}

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Equipment {
    None,
    Dumbbell,
    Kettlebell,
    MedicineBall,
    ResistanceBand,
    JumpRope,
    Mat,
    Bench,
    Box,
    Wall,
    Chair,
    PullUpBar,
    Rings,
    TRX,
    Bosu,
    SwissBall,
    FoamRoller,
}

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Body {
    Full,
    Core,
    Abs,
    Pectorals,
    Legs,
    Hip,
    Shoulder,
    Buttocks,
}

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mouvement {
    Coordination,
    Balance,
    Rotation,
    Stamina,
    Strength,
    Stationary,
    Stretching,
    Dynamic,
    Footwork,
}

impl Mouvement {
    pub fn icon(&self) -> char {
        match self {
            Self::Balance => '🛹',
            Self::Coordination => '🤹',
            Self::Rotation => '💫',
            Self::Stamina => '💓',
            Self::Strength => '💪',
            Self::Stationary => '🙌',
            Self::Stretching => '🪢',
            Self::Dynamic => '🏃',
            Self::Footwork => '👣',
        }
    }
}
