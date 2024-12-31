use derive_more::{Deref, DerefMut, Display, From, IntoIterator};
use serde::{Deserialize, Serialize};
use slug::slugify;

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash, From, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tag {
    Rest,
    HiiT,
    WarmUp,
    Boxing,
    #[from]
    #[serde(untagged)]
    Difficulty(Difficulty),
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
    pub fn icon(&self) -> String {
        match self {
            Self::Rest => "💤".to_owned(),
            Self::Squat(squat) => squat.to_string(),
            Self::Difficulty(difficulty) => difficulty.icon(),
            Self::Body(body) => body.to_string(),
            Self::Equipment(equipment) => equipment.to_string(),
            Self::Mouvement(mouvement) => mouvement.icon(),
            Self::HiiT => "🧨".to_owned(),
            Self::WarmUp => "🔥".to_owned(),
            Self::Boxing => "🥊".to_owned(),
            Self::Plank(plank) => plank.to_string(),
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
    pub fn icon(&self) -> String {
        match self {
            Self::Easy => "🟩".to_owned(),
            Self::Medium => "🟨".to_owned(),
            Self::Hard => "🟧".to_owned(),
            Self::Elite => "🟥".to_owned(),
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
    Barbell,
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
}

#[derive(Display, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mouvement {
    Coordination,
    Balance,
    Rotation,
    Stamina,
    Strengthening,
    Stationary,
    Stretching,
    Dynamic,
    Footwork,
}

impl Mouvement {
    pub fn icon(&self) -> String {
        match self {
            Self::Balance => "🛹".to_owned(),
            Self::Coordination => "🤹🏼".to_owned(),
            Self::Rotation => "💫".to_owned(),
            Self::Stamina => "💓".to_owned(),
            Self::Strengthening => "🏋".to_owned(),
            Self::Stationary => "🤸🏼".to_owned(),
            Self::Stretching => "🪢".to_owned(),
            Self::Dynamic => "🏃".to_owned(),
            Self::Footwork => "👣".to_owned(),
        }
    }
}
