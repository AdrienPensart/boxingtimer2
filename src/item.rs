use crate::tag::{Body, Difficulty, Mouvement, Tag, Tags};
use crate::workout::Workout;
use bon::Builder;
use itertools::Itertools;
use slug::slugify;

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash, Builder)]
pub struct Item {
    #[builder(into)]
    name: String,
    #[builder(default, into)]
    tags: Tags,
    #[builder(default, into)]
    variations: Vec<String>,
    #[builder(into)]
    description: Option<String>,
    #[builder(into)]
    difficulty: Option<Difficulty>,
}

impl Item {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn slug(&self) -> String {
        slugify(&self.name)
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
            let joined = self.tags.iter().map(|&t| t.to_string()).join(" / ");
            if !joined.is_empty() {
                return Some(joined);
            }
        }
        None
    }
    pub fn workout(&self, duration: std::time::Duration) -> Workout {
        Workout::builder()
            .item(self.clone())
            .stopwatch(duration)
            .build()
    }
    pub fn easy(&self, duration: std::time::Duration) -> Workout {
        Workout::builder()
            .item(self.clone())
            .stopwatch(duration)
            .difficulty(Difficulty::Easy)
            .build()
    }
    pub fn medium(&self, duration: std::time::Duration) -> Workout {
        Workout::builder()
            .item(self.clone())
            .stopwatch(duration)
            .difficulty(Difficulty::Medium)
            .build()
    }
    pub fn hard(&self, duration: std::time::Duration) -> Workout {
        Workout::builder()
            .item(self.clone())
            .stopwatch(duration)
            .difficulty(Difficulty::Hard)
            .build()
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// GENERIC

pub static WARM_UP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("🔥Warm Up")
        .description("Generic warm-up")
        .build()
});

pub static WORKOUT: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("🎯Workout")
        .description("Generic workout")
        .build()
});

pub static REST: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("💤Rest")
        .tags(bon::vec![Mouvement::Rest])
        .build()
});

pub static WALK: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::builder().name("Walk").build());

pub static RUN: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::builder().name("Run").build());

pub static SIDE_STEPS: std::sync::LazyLock<Item> =
    std::sync::LazyLock::new(|| Item::builder().name("Side Steps").build());

// PLANKS

pub static PLANK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Plank")
        .tags(bon::vec![Mouvement::Stationary, Body::Core])
        .build()
});

pub static PLANK_SHOULDER_TAP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Plank Shoulder Tap")
        .tags(bon::vec![Mouvement::Dynamic])
        .build()
});

pub static SIDE_PLANK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Side Plank")
        .tags(bon::vec![Mouvement::Stationary])
        .build()
});

pub static COMMANDO_PLANK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Commando Plank")
        .tags(bon::vec![Mouvement::Dynamic])
        .build()
});

// WARM UP

pub static HEAD_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Head Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .build()
});

pub static SHOULDER_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Shoulder Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .build()
});

pub static ARM_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Arms Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .build()
});

pub static ELBOW_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Elbows Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .build()
});

pub static WRIST_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Wrists Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .build()
});

pub static HIP_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Hips Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .build()
});

pub static KNEE_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Knees Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .build()
});

pub static FEET_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Feet Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .build()
});

pub static HEEL_RAISES: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Heels Raises")
        .tags(bon::vec![Tag::WarmUp])
        .build()
});

pub static INCHWORM: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Inchworm")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Stretching])
        .build()
});

pub static LEG_SWINGS: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Leg Swings")
        .tags(bon::vec![Tag::WarmUp])
        .variations(bon::vec!["Side Leg Swings"])
        .build()
});

pub static SIDE_LEG_SWINGS: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Side Leg Swings")
        .tags(bon::vec![Tag::WarmUp])
        .variations(bon::vec!["Front Leg Swings"])
        .build()
});

pub static SINGLE_LEG_TOUCH_TOES: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Single Leg Touch Toes")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Stretching])
        .build()
});

pub static WINDMILL: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Windmill")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Stretching])
        .build()
});

pub static BUTT_KICKS: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Butt Kicks")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
        .build()
});

pub static HIGH_KNEES: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("High Knees")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
        .build()
});

pub static JUMPING_JACK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Jumping Jack")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
        .build()
});

pub static MOUNTAIN_CLIMBER: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Mountain Climber")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
        .build()
});

pub static SQUAT: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Squat")
        .variations(bon::vec![
            "Jump Squat",
            "Sumo Squat",
            "Single Leg Squat",
            "Bulgarian Squat"
        ])
        .tags(bon::vec![Mouvement::Dynamic])
        .build()
});

pub static JUMP_SQUAT: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Jump Squat")
        .tags(bon::vec![Mouvement::Dynamic])
        .build()
});

pub static PUSH_UP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Push Up")
        .tags(bon::vec![Mouvement::Dynamic])
        .build()
});

pub static PULL_UP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Pull Up")
        .tags(bon::vec![Mouvement::Dynamic])
        .build()
});

pub static SPEED_STEP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Speed Step")
        .tags(bon::vec![Mouvement::Dynamic])
        .build()
});

pub static ALTERNATE_STEP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Alternate Step")
        .tags(bon::vec![Mouvement::Dynamic])
        .build()
});

pub static LUNGE: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Lunge")
        .variations(bon::vec!["Reverse Lunge", "Walking Lunge", "Jump Lunge"])
        .tags(bon::vec![Mouvement::Dynamic])
        .build()
});

pub static BURPEE: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Burpee")
        .tags(bon::vec![Mouvement::Dynamic])
        .variations(bon::vec!["Navy Seal Burpee (push-up)"])
        .build()
});

pub static JUMPS: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Jumps")
        .variations(bon::vec!["Forward Jumps"])
        .tags(bon::vec![Mouvement::Dynamic])
        .build()
});

pub static CRUNCHES: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Crunches")
        .tags([Tag::Body(Body::Abs)])
        .build()
});

pub static SCISSOR_KICK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Scissor Kick")
        .tags(bon::vec![Body::Abs, Body::Hip])
        .build()
});

pub static HIP_THRUST: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Hip Thrust")
        .tags(bon::vec![Body::Hip])
        .build()
});

pub static BOXING_ROUND: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("🥊Boxing Round")
        .tags(bon::vec![Tag::Boxing])
        .build()
});

pub static ROPE: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Rope")
        .variations(bon::vec!["Double Unders", "Criss Cross", "High Knees"])
        .build()
});
