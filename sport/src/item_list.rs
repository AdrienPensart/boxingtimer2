use crate::{
    item::Item,
    tag::{Body, Boxing, Difficulty, Equipment, Mouvement, Tag},
    workout::Workout,
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use strum::VariantArray;
use strum_macros::VariantArray;

#[derive(Clone, VariantArray)]
pub enum ItemList {
    WarmUp,
    Workout,
    Tabata,
    Rest,
    Walk,
    Run,
    Sprint,
    SideSteps,
    Plank,
    PlankShoulderTap,
    SidePlank,
    CommandoPlank,
    HeadRotation,
    ShoulderRotation,
    ArmRotation,
    ElbowRotation,
    WristRotation,
    HipRotation,
    KneeRotation,
    AnkleRotation,
    HeelRaise,
    Inchworm,
    LegSwingFront,
    LegSwingSide,
    LegTouchToe,
    Windmill,
    ButtKicks,
    HighKnees,
    JumpingJack,
    MountainClimber,
    Squat,
    SquatBulgarian,
    SquatSingleLeg,
    SquatJump,
    SquatSumo,
    PushUp,
    PullUp,
    SpeedStep,
    SkatingStep,
    AlternateStepUp,
    Lunge,
    LungeReverse,
    LungeJumping,
    LungeWalking,
    Burpee,
    BurpeePushUp,
    BurpeeNavySeal,
    Jump,
    JumpForward,
    BoxJump,
    Crunch,
    ScissorKick,
    HipThrust,
    BoxingRound,
    JumpRope,
    JumpRopeDoubleUnders,
    JumpRopeCrissCross,
    JumpRopeHighKnees,
}

impl ItemList {
    pub fn export_json() -> String {
        serde_json::to_string_pretty(&Self::items()).unwrap()
    }
    pub fn tag_to_items() -> HashMap<Tag, HashSet<Item>> {
        let mut tags2items: HashMap<Tag, HashSet<Item>> = HashMap::new();
        for item in Self::items() {
            for tag in item.tags().iter() {
                tags2items.entry(*tag).or_default().insert(item.clone());
            }
        }
        tags2items
    }
    pub fn tags() -> Vec<Tag> {
        let mut tags = Self::tag_to_items().keys().cloned().collect_vec();
        tags.sort_by_key(|t| t.slug());
        tags
    }
    pub fn items() -> Vec<Item> {
        let mut items: Vec<_> = Self::VARIANTS
            .iter()
            .map(|il| Into::<Item>::into(il.clone()))
            .collect();
        items.sort_by_key(|s| s.name().to_string());
        items
    }
    pub fn workout(self, duration: std::time::Duration) -> Workout {
        let item: Item = self.into();
        item.workout(duration)
    }
    pub fn difficulty(self, duration: std::time::Duration, difficulty: Difficulty) -> Workout {
        let item: Item = self.into();
        Workout::builder()
            .item(item)
            .stopwatch(duration)
            .difficulty(difficulty)
            .build()
    }
    pub fn easy(self, duration: std::time::Duration) -> Workout {
        self.difficulty(duration, Difficulty::Easy)
    }
    pub fn medium(self, duration: std::time::Duration) -> Workout {
        self.difficulty(duration, Difficulty::Medium)
    }
    pub fn hard(self, duration: std::time::Duration) -> Workout {
        self.difficulty(duration, Difficulty::Hard)
    }
    pub fn elite(self, duration: std::time::Duration) -> Workout {
        self.difficulty(duration, Difficulty::Elite)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Item> for ItemList {
    fn into(self) -> Item {
        match self {
            Self::WarmUp => Item::builder()
                .name("🔥Warm Up")
                .tags(bon::vec![Tag::WarmUp])
                .description("Generic warm-up")
                .build(),
            Self::Workout => Item::builder()
                .name("Workout")
                .icon('🎯')
                .description("Generic workout")
                .build(),
            Self::Tabata => Item::builder()
                .name("Workout")
                .tags(bon::vec![Tag::HiiT])
                .icon('🧨')
                .description("Generic workout")
                .build(),
            Self::Rest => Item::builder()
                .name("Rest")
                .icon('💤')
                .tags(bon::vec![Tag::Rest])
                .build(),
            Self::Walk => Item::builder()
                .name("Walk")
                .icon('🧍')
                .tags(bon::vec![Mouvement::Footwork])
                .difficulty(Difficulty::Easy)
                .build(),
            Self::Run => Item::builder()
                .name("Run")
                .tags(bon::vec![Mouvement::Footwork])
                .difficulty(Difficulty::Medium)
                .icon('🏃')
                .build(),
            Self::Sprint => Item::builder()
                .name("Sprint")
                .tags(bon::vec![Mouvement::Footwork])
                .difficulty(Difficulty::Hard)
                .icon('🏃')
                .build(),
            Self::SideSteps => Item::builder()
                .name("Side Steps")
                .tags(bon::vec![Mouvement::Footwork])
                .icon('👯')
                .build(),
            Self::Plank => Item::builder()
                .name("Plank")
                .tags(bon::vec![Mouvement::Stationary, Body::Core, Body::Shoulder])
                .icon('🚪')
                .build(),
            Self::PlankShoulderTap => Item::builder()
                .name("Plank Shoulder Tap")
                .tags(bon::vec![Mouvement::Dynamic, Body::Core, Body::Shoulder])
                .icon('🚪')
                .build(),
            Self::SidePlank => Item::builder()
                .name("Side Plank")
                .tags(bon::vec![Mouvement::Stationary, Body::Obliques])
                .icon('🚪')
                .build(),
            Self::CommandoPlank => Item::builder()
                .name("Commando Plank")
                .tags(bon::vec![Mouvement::Dynamic])
                .icon('🚪')
                .build(),
            Self::HeadRotation => Item::builder()
                .name("Head Rotation")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
                .icon('⚙')
                .build(),
            Self::ShoulderRotation => Item::builder()
                .name("Shoulder Rotation")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
                .icon('⚙')
                .build(),
            Self::ArmRotation => Item::builder()
                .name("Arms Rotation")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
                .icon('⚙')
                .build(),
            Self::ElbowRotation => Item::builder()
                .name("Elbows Rotation")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
                .icon('⚙')
                .build(),
            Self::WristRotation => Item::builder()
                .name("Wrists Rotation")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
                .icon('⚙')
                .build(),
            Self::HipRotation => Item::builder()
                .name("Hips Rotation")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
                .icon('⚙')
                .build(),
            Self::KneeRotation => Item::builder()
                .name("Knees Rotation")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
                .icon('⚙')
                .build(),
            Self::AnkleRotation => Item::builder()
                .name("Feet Rotation")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
                .icon('⚙')
                .build(),
            Self::HeelRaise => Item::builder()
                .name("Heels Raises")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Stationary])
                .icon('⚙')
                .build(),
            Self::Inchworm => Item::builder()
                .name("Inchworm")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Stretching])
                .icon('⚙')
                .build(),
            Self::LegSwingFront => Item::builder()
                .name("Leg Swings Front")
                .tags(bon::vec![Tag::WarmUp])
                .icon('⚙')
                .build(),
            Self::LegSwingSide => Item::builder()
                .name("Leg Swings Side")
                .tags(bon::vec![Tag::WarmUp])
                .icon('⚙')
                .build(),
            Self::LegTouchToe => Item::builder()
                .name("Single Leg Touch Toes")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Stretching])
                .icon('⚙')
                .build(),
            Self::Windmill => Item::builder()
                .name("Windmill")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Stretching])
                .icon('⚙')
                .build(),
            Self::ButtKicks => Item::builder()
                .name("Butt Kicks")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
                .icon('⚙')
                .build(),
            Self::HighKnees => Item::builder()
                .name("High Knees")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
                .icon('⚙')
                .build(),
            Self::JumpingJack => Item::builder()
                .name("Jumping Jack")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
                .icon('💓')
                .build(),
            Self::MountainClimber => Item::builder()
                .name("Mountain Climber")
                .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
                .icon('💓')
                .build(),
            Self::Squat => Item::builder()
                .name("Squat")
                .tags(bon::vec![Mouvement::Dynamic])
                .icon('💓')
                .build(),
            Self::SquatBulgarian => Item::builder()
                .name("Squat Bulgarian")
                .tags(bon::vec![Mouvement::Dynamic])
                .icon('💓')
                .build(),
            Self::SquatSingleLeg => Item::builder()
                .name("Squat Single Leg")
                .tags(bon::vec![Mouvement::Dynamic])
                .icon('💓')
                .build(),
            Self::SquatJump => Item::builder()
                .name("Squat Jump")
                .tags(bon::vec![Mouvement::Dynamic])
                .icon('💓')
                .build(),
            Self::SquatSumo => Item::builder()
                .name("Squat Sumo")
                .tags(bon::vec![Mouvement::Dynamic])
                .icon('💓')
                .build(),
            Self::PushUp => Item::builder()
                .name("Push Up")
                .tags(bon::vec![Mouvement::Dynamic, Mouvement::Strength])
                .icon('💓')
                .build(),
            Self::PullUp => Item::builder()
                .name("Pull Up")
                .tags(bon::vec![Mouvement::Dynamic, Mouvement::Strength])
                .icon('💓')
                .build(),
            Self::SpeedStep => Item::builder()
                .name("Speed Step")
                .tags(bon::vec![Mouvement::Dynamic, Mouvement::Footwork])
                .icon('💓')
                .build(),
            Self::SkatingStep => Item::builder()
                .name("Skating Step")
                .tags(bon::vec![Mouvement::Dynamic, Mouvement::Footwork])
                .icon('💓')
                .build(),
            Self::AlternateStepUp => Item::builder()
                .name("Alternate Step")
                .tags(bon::vec![Mouvement::Dynamic, Mouvement::Footwork])
                .icon('💓')
                .build(),
            Self::Lunge => Item::builder()
                .name("Lunge")
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Mouvement::Balance,
                    Mouvement::Strength
                ])
                .icon('💓')
                .build(),
            Self::LungeReverse => Item::builder()
                .name("Lunge Reverse")
                .icon('💓')
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Mouvement::Balance,
                    Mouvement::Strength
                ])
                .build(),
            Self::LungeJumping => Item::builder()
                .name("Lunge Jump")
                .icon('💓')
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Mouvement::Balance,
                    Mouvement::Strength
                ])
                .difficulty(Difficulty::Medium)
                .build(),
            Self::LungeWalking => Item::builder()
                .name("Lunge Walking")
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Mouvement::Balance,
                    Mouvement::Strength
                ])
                .icon('💓')
                .build(),
            Self::Burpee => Item::builder()
                .name("Burpee")
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Body::Full,
                    Mouvement::Stamina
                ])
                .icon('💓')
                .difficulty(Difficulty::Medium)
                .build(),
            Self::BurpeePushUp => Item::builder()
                .name("Burpee Push Up")
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Body::Full,
                    Mouvement::Stamina
                ])
                .icon('💓')
                .build(),
            Self::BurpeeNavySeal => Item::builder()
                .name("Burpee Navy Seal")
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Body::Full,
                    Mouvement::Stamina
                ])
                .icon('💓')
                .build(),
            Self::Jump => Item::builder()
                .name("Jumps")
                .tags(bon::vec![Mouvement::Dynamic, Mouvement::Stamina])
                .icon('💓')
                .build(),
            Self::JumpForward => Item::builder()
                .name("Jumps Forward")
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Mouvement::Stamina,
                    Mouvement::Balance
                ])
                .icon('💓')
                .build(),
            Self::BoxJump => Item::builder()
                .name("Box Jump")
                .tags(bon::vec![Body::Abs, Body::Core, Equipment::Box])
                .icon('💓')
                .build(),
            Self::Crunch => Item::builder()
                .name("Crunches")
                .tags(bon::vec![Body::Abs, Body::Core])
                .icon('💓')
                .build(),
            Self::ScissorKick => Item::builder()
                .name("Scissor Kick")
                .tags(bon::vec![Body::Abs, Body::Hip, Body::Core])
                .icon('💓')
                .build(),
            Self::HipThrust => Item::builder()
                .name("Hip Thrust")
                .tags(bon::vec![Body::Hip, Body::Core, Body::Buttocks])
                .icon('💓')
                .build(),
            Self::BoxingRound => Item::builder()
                .name("Boxing Round")
                .icon('🥊')
                .tags(bon::vec![Boxing::Round, Mouvement::Stamina, Body::Full])
                .build(),
            Self::JumpRope => Item::builder()
                .name("Jump Rope")
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Mouvement::Coordination,
                    Equipment::JumpRope,
                ])
                .icon('🪱')
                .build(),
            Self::JumpRopeDoubleUnders => Item::builder()
                .name("Jump Rope Double Unders")
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Mouvement::Coordination,
                    Equipment::JumpRope,
                ])
                .icon('🪱')
                .difficulty(Difficulty::Medium)
                .build(),
            Self::JumpRopeCrissCross => Item::builder()
                .name("Jump Rope Criss Cross")
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Mouvement::Coordination,
                    Equipment::JumpRope,
                ])
                .difficulty(Difficulty::Medium)
                .icon('🪱')
                .build(),
            Self::JumpRopeHighKnees => Item::builder()
                .name("Jump Rope High Knees")
                .tags(bon::vec![
                    Mouvement::Dynamic,
                    Mouvement::Coordination,
                    Equipment::JumpRope,
                ])
                .difficulty(Difficulty::Medium)
                .icon('🪱')
                .build(),
        }
    }
}
