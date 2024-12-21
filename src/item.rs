use crate::tag::{Body, Difficulty, Mouvement, Tag, Tags};
use crate::workout::Workout;
use bon::Builder;

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash, Builder)]
pub struct Item {
    name: String,
    tags: Tags,
    #[builder(into)]
    description: Option<String>,
}

impl Item {
    pub fn new(name: &str, tags: &[Tag], description: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            tags: tags.into(),
            description,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn tags(&self) -> &Tags {
        &self.tags
    }
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn is_rest(&self) -> bool {
        self.tags.contains(&Mouvement::Rest.into())
    }
    pub fn joined_tags(&self) -> Option<String> {
        if !self.tags.is_empty() {
            Some(
                self.tags
                    .iter()
                    .map(|&t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" / "),
            )
        } else {
            None
        }
    }
    pub fn workout(&self, duration: std::time::Duration) -> Workout {
        Workout::new(self, duration, None)
    }
    pub fn easy(&self, duration: std::time::Duration) -> Workout {
        Workout::new(self, duration, Some(Difficulty::Easy))
    }
    pub fn medium(&self, duration: std::time::Duration) -> Workout {
        Workout::new(self, duration, Some(Difficulty::Medium))
    }
    pub fn hard(&self, duration: std::time::Duration) -> Workout {
        Workout::new(self, duration, Some(Difficulty::Hard))
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// GENERIC

pub static WARM_UP: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Warm Up", &[], Some("Generic warm-up".to_owned())));

pub static WORKOUT: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Workout", &[], Some("Generic workout".to_owned())));

pub static REST: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Rest", &[Mouvement::Rest.into()], None));

// PLANKS

pub static PLANK: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Plank", &[Mouvement::Stationary.into()], None));

pub static SIDE_PLANK: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Side Plank", &[Mouvement::Stationary.into()], None));

pub static COMMANDO_PLANK: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Commando Plank", &[Mouvement::Stationary.into()], None));

// WARM UP

pub static HEAD_ROTATION: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Head Rotation", &[], None));

pub static SHOULDER_ROTATION: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Shoulder Rotation", &[], None));

pub static ARM_ROTATION: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Arms Rotation", &[], None));

pub static ELBOW_ROTATION: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Elbows Rotation", &[], None));

pub static WRIST_ROTATION: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Wrists Rotation", &[], None));

pub static HIP_ROTATION: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Hips Rotation", &[], None));

pub static KNEE_ROTATION: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Knees Rotation", &[], None));

pub static FEET_ROTATION: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Feet Rotation", &[], None));

pub static HEEL_RAISES: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Heels Raises", &[], None));

pub static LEG_SWINGS: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Leg Swings", &[], None));

pub static SIDE_LEG_SWINGS: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Side Leg Swings", &[], None));

pub static SINGLE_LEG_TOUCH_TOES: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::new(
        "Single Leg Touch Toes",
        &[Mouvement::Stretching.into()],
        None,
    )
});

pub static WINDMILL: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Windmill", &[Mouvement::Stretching.into()], None));

pub static BUTT_KICKS: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Butt Kicks", &[Mouvement::Dynamic.into()], None));

pub static HIGH_KNEES: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("High Knees", &[Mouvement::Dynamic.into()], None));

pub static JUMPING_JACK: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Jumping Jack", &[Mouvement::Dynamic.into()], None));

pub static MOUNTAIN_CLIMBER: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Mountain Climber", &[Mouvement::Dynamic.into()], None));

pub static SQUAT: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Squat", &[Mouvement::Dynamic.into()], None));

pub static JUMP_SQUAT: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Jump Squat", &[Mouvement::Dynamic.into()], None));

pub static PUSH_UP: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Push Up", &[Mouvement::Dynamic.into()], None));

pub static PULL_UP: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Pull Up", &[Mouvement::Dynamic.into()], None));

pub static SPEED_STEP: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Speed Step", &[Mouvement::Dynamic.into()], None));

pub static ALTERNATE_STEP: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Alternate Step", &[Mouvement::Dynamic.into()], None));

pub static LUNGE: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Lunge", &[Mouvement::Dynamic.into()], None));

pub static ALTERNATE_LUNGE: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Alternate Lunge", &[Mouvement::Dynamic.into()], None));

pub static BURPEE: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Burpee", &[Mouvement::Dynamic.into()], None));

pub static JUMPS: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Jumps", &[Mouvement::Dynamic.into()], None));

pub static CRUNCHES: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Crunches", &[Tag::Body(Body::Abs)], None));

pub static SCISSOR_KICK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::new(
        "Scissor Kick",
        &[Tag::Body(Body::Abs), Tag::Body(Body::Hip)],
        None,
    )
});

pub static HIP_THRUST: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Hip Thrust", &[Tag::Body(Body::Hip)], None));

pub static BOXING_ROUND: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::new("Boxing Round 🥊", &[], None));
