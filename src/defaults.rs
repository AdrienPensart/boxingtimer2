use crate::duration::{MINUTE, SECOND};
use crate::exercises::Exercises;
use crate::item::Item;
use crate::sequence::{Sequence, ROUNDS};
use crate::sound::Sound;
use crate::tag::{Body, Difficulty, Equipment, Mouvement, Tag};

pub const DEFAULT_ICON: char = '🎯';
pub const DEFAULT_INTERVAL: u32 = 1000;
pub const PREPARE: u64 = 15;
pub const PREPARE_DURATION: std::time::Duration = std::time::Duration::from_secs(PREPARE);
pub const PREPARE_LABEL: &str = "⏳";
pub const RESTART_SEQUENCE: &str = "♼";
pub const PREVIOUS_ITEM: &str = "⏪";
pub const NEXT_ITEM: &str = "⏩";
pub const RANDOMIZE: &str = "🎲";
pub const SIGNAL: &str = "🛎";

pub fn default_sequences() -> Vec<Sequence> {
    let warmup = Sequence::simple()
        .name("Warm-up (articular)")
        .workouts(&[
            // 1 minute
            HEAD_ROTATION.easy(20 * SECOND),
            SHOULDER_ROTATION.easy(20 * SECOND),
            ARM_ROTATION.easy(20 * SECOND),
            // 1 minute
            ELBOW_ROTATION.easy(20 * SECOND),
            WRIST_ROTATION.easy(20 * SECOND),
            HIP_ROTATION.easy(20 * SECOND),
            // 1 minute
            KNEE_ROTATION.easy(20 * SECOND),
            FEET_ROTATION.easy(20 * SECOND),
            HEEL_RAISES.easy(20 * SECOND),
            // 1 minute
            LEG_SWINGS_FRONT.easy(20 * SECOND),
            LEG_SWINGS_SIDE.easy(20 * SECOND),
            SINGLE_LEG_TOUCH_TOES.easy(20 * SECOND),
            // 1 minute
            BUTT_KICKS.easy(30 * SECOND),
            HIGH_KNEES.easy(30 * SECOND),
            // 1 minute
            JUMPING_JACK.easy(30 * SECOND),
            MOUNTAIN_CLIMBER.easy(30 * SECOND),
            // 1 minute
            SQUAT.medium(30 * SECOND),
            PUSH_UP.medium(30 * SECOND),
            // 1 minute
            SPEED_STEP.easy(30 * SECOND),
            ALTERNATE_STEP.easy(30 * SECOND),
            // 1 minute
            LUNGE.medium(30 * SECOND),
            BURPEE.medium(30 * SECOND),
        ])
        .difficulty(Difficulty::Easy)
        .sound(&Sound::Silent)
        .icon('🔥')
        .call()
        .register();

    let cardio_warmup = Sequence::simple()
        .name("Warm-up (cardio)")
        .workouts(&[
            // 1 minute
            JUMPING_JACK.easy(15 * SECOND),
            HIGH_KNEES.easy(15 * SECOND),
            JUMPING_JACK.easy(15 * SECOND),
            BUTT_KICKS.easy(15 * SECOND),
            // 1 minute
            JUMPING_JACK.easy(15 * SECOND),
            ALTERNATE_STEP.easy(15 * SECOND),
            JUMPING_JACK.easy(15 * SECOND),
            MOUNTAIN_CLIMBER.easy(15 * SECOND),
            // 1 minute
            JUMPING_JACK.easy(15 * SECOND),
            SQUAT.easy(15 * SECOND),
            JUMPING_JACK.easy(15 * SECOND),
            LUNGE.easy(15 * SECOND),
            // 1 minute
            JUMPING_JACK.easy(15 * SECOND),
            SQUAT_JUMP.easy(15 * SECOND),
            JUMPING_JACK.easy(15 * SECOND),
            LUNGE.medium(15 * SECOND),
            // 1 minute
            JUMPING_JACK.easy(15 * SECOND),
            JUMPS.easy(15 * SECOND),
            JUMPING_JACK.easy(15 * SECOND),
            PUSH_UP.easy(15 * SECOND),
            // 1 minute
            JUMPING_JACK.easy(15 * SECOND),
            COMMANDO_PLANK.easy(15 * SECOND),
            JUMPING_JACK.easy(15 * SECOND),
            BURPEE.easy(15 * SECOND),
        ])
        .difficulty(Difficulty::Easy)
        .icon('💓')
        .sound(&Sound::Beep)
        .call()
        .register();

    let _5_rounds_1m = Sequence::rounds()
        .name("1m/5x Workout")
        .rounds(5)
        .workout(WORKOUT.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .sound(&Sound::Beep)
        .difficulty(Difficulty::Medium)
        .icon('🥊')
        .call()
        .register();

    let _10_rounds_1m = Sequence::rounds()
        .name("1m/10x Workout")
        .rounds(10)
        .workout(WORKOUT.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .sound(&Sound::Beep)
        .difficulty(Difficulty::Medium)
        .icon('🥊')
        .call()
        .register();

    let _15_rounds_1m = Sequence::rounds()
        .name("1m/15x Workout")
        .rounds(10)
        .workout(WORKOUT.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .sound(&Sound::Beep)
        .icon('🥊')
        .difficulty(Difficulty::Medium)
        .call()
        .register();

    let boxing_3x2m_30s = Sequence::rounds()
        .name("3x2m")
        .rounds(3 * ROUNDS)
        .workout(BOXING_ROUND.workout(2 * MINUTE))
        .rest(30 * SECOND)
        .sound(&Sound::Bell)
        .icon('🥊')
        .difficulty(Difficulty::Easy)
        .call()
        .register();

    let boxing_3x3m_1m = Sequence::rounds()
        .name("3x3m")
        .rounds(3 * ROUNDS)
        .workout(BOXING_ROUND.workout(2 * MINUTE))
        .rest(MINUTE)
        .sound(&Sound::Bell)
        .icon('🥊')
        .difficulty(Difficulty::Easy)
        .call()
        .register();

    let boxing_6x2m_30s = Sequence::rounds()
        .name("6x2m")
        .rounds(6 * ROUNDS)
        .workout(BOXING_ROUND.workout(2 * MINUTE))
        .rest(30 * SECOND)
        .sound(&Sound::Bell)
        .icon('🥊')
        .difficulty(Difficulty::Medium)
        .call()
        .register();

    let boxing_6x3m_1m = Sequence::rounds()
        .name("6x3m")
        .rounds(6 * ROUNDS)
        .workout(BOXING_ROUND.workout(3 * MINUTE))
        .rest(MINUTE)
        .sound(&Sound::Bell)
        .icon('🥊')
        .difficulty(Difficulty::Hard)
        .call()
        .register();

    let boxing_12x3m_1m = Sequence::rounds()
        .name("12x3m")
        .rounds(12 * ROUNDS)
        .workout(BOXING_ROUND.workout(3 * MINUTE))
        .rest(MINUTE)
        .sound(&Sound::Bell)
        .icon('🥊')
        .difficulty(Difficulty::Elite)
        .call()
        .register();

    let stamina_jab_cross_hook = Sequence::repeat()
        .name("Stamina 30s 1-2-3")
        .description("1 | 2 | 1-2 | 1-2-3")
        .exercises(Exercises::from(
            vec![
                "Jab (1)",
                "Cross (2)",
                "Jab | Cross (1-2)",
                "Jab | Cross | Hook (1-2-3)",
            ],
            Some('🥊'),
        ))
        .workout(30 * SECOND)
        .icon('🥊')
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Medium)
        .sound(&Sound::Bell)
        .call()
        .register();

    let stamina_jab_jab_cross_cross = Sequence::repeat()
        .name("Stamina 30s 1-1-2-2")
        .description("1 | 1-1 | 1-1-2 | 1-1-2-2")
        .exercises(Exercises::from(
            vec![
                "Jab (1)",
                "Double Jab (1-1)",
                "Double Jab | Cross (1-1-2)",
                "Double Jab | Cross | Cross",
            ],
            Some('🥊'),
        ))
        .workout(30 * SECOND)
        .icon('🥊')
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Medium)
        .sound(&Sound::Bell)
        .call()
        .register();

    let stamina_jab_cross_hook_cross = Sequence::repeat()
        .name("Stamina 30s 1-2-3-2")
        .description("1 | 1-2 | 1-2-3 | 1-2-3-2")
        .exercises(Exercises::from(
            vec![
                "Jab (1)",
                "Jab | Cross (1-2)",
                "Jab | Cross | Hook (1-2-3)",
                "Jab | Cross | Hook | Cross (1-2-3-2)",
            ],
            Some('🥊'),
        ))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .icon('🥊')
        .rest(MINUTE)
        .difficulty(Difficulty::Medium)
        .sound(&Sound::Bell)
        .call()
        .register();

    let stamina_roll_left = Sequence::repeat()
        .name("Stamina 30s 1-2-3-ROLL-3-2")
        .description("1-2-3 | 1-2-3-ROLL | 1-2-3-ROLL-3 | 1-2-3-ROLL-3-2")
        .exercises(Exercises::from(
            vec![
                "Jab | Cross | Hook",
                "Jab | Cross | Hook | ROLL",
                "Jab | Cross | Hook | ROLL | Hook",
                "Jab | Cross | Hook | ROLL | Hook | Cross",
            ],
            Some('🥊'),
        ))
        .workout(30 * SECOND)
        .icon('🥊')
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Hard)
        .sound(&Sound::Bell)
        .call()
        .register();

    let stamina_roll_right = Sequence::repeat()
        .name("Stamina 30s 1-2-ROLL-2-3-2")
        .description("1-2-ROLL | 1-2-ROLL-2 | 1-2-ROLL-2-3 | 1-2-ROLL-2-3-2")
        .exercises(Exercises::from(
            vec![
                "Jab | Cross | ROLL",
                "Jab | Cross | ROLL | Cross",
                "Jab | Cross | ROLL | Cross | Hook",
                "Jab | Cross | ROLL | Cross | Hook | Cross",
            ],
            Some('🥊'),
        ))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .icon('🥊')
        .difficulty(Difficulty::Hard)
        .sound(&Sound::Bell)
        .call()
        .register();

    let random_training = Sequence::random()
        .name("Random training (30s)")
        .workouts(&[
            JUMPING_JACK.workout(30 * SECOND),
            PULL_UP.workout(30 * SECOND),
            PLANK.workout(30 * SECOND),
            SIDE_PLANK.workout(30 * SECOND),
            SQUAT_JUMP.workout(30 * SECOND),
            BURPEE.workout(30 * SECOND),
            PUSH_UP.workout(30 * SECOND),
            LUNGE.workout(30 * SECOND),
            CRUNCHES.workout(30 * SECOND),
        ])
        .icon('🎲')
        .rest(30 * SECOND)
        .sound(&Sound::Beep)
        .call()
        .register();

    let hiit_4x = Sequence::rounds()
        .name("Half-Tabata 20s (4x)")
        .rounds(4 * ROUNDS)
        .workout(TABATA.workout(20 * SECOND))
        .difficulty(Difficulty::Medium)
        .icon('🧨')
        .rest(10 * SECOND)
        .sound(&Sound::Beep)
        .call();

    let hiit_8x = Sequence::rounds()
        .name("Tabata 20s (8x)")
        .rounds(8 * ROUNDS)
        .workout(TABATA.workout(20 * SECOND))
        .difficulty(Difficulty::Hard)
        .icon('🧨')
        .rest(10 * SECOND)
        .sound(&Sound::Beep)
        .call()
        .register();

    let _1mn = WORKOUT
        .workout(1 * MINUTE)
        .sequence(&Sound::Bell)
        .register();
    let _2mn = WORKOUT
        .workout(2 * MINUTE)
        .sequence(&Sound::Bell)
        .register();
    let _3mn = WORKOUT
        .workout(3 * MINUTE)
        .sequence(&Sound::Bell)
        .register();
    let _5mn = WORKOUT
        .workout(5 * MINUTE)
        .sequence(&Sound::Bell)
        .register();
    let _10mn = WORKOUT
        .workout(10 * MINUTE)
        .sequence(&Sound::Bell)
        .register();
    let _15mn = WORKOUT
        .workout(15 * MINUTE)
        .sequence(&Sound::Bell)
        .register();

    vec![
        warmup,
        cardio_warmup,
        _5_rounds_1m,
        _10_rounds_1m,
        _15_rounds_1m,
        boxing_3x2m_30s,
        boxing_3x3m_1m,
        boxing_6x2m_30s,
        boxing_6x3m_1m,
        boxing_12x3m_1m,
        stamina_jab_cross_hook,
        stamina_jab_jab_cross_cross,
        stamina_jab_cross_hook_cross,
        stamina_roll_left,
        stamina_roll_right,
        hiit_4x,
        hiit_8x,
        random_training,
        _1mn,
        _2mn,
        _3mn,
        _5mn,
        _10mn,
        _15mn,
    ]
}

pub static WARM_UP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("🔥Warm Up")
        .description("Generic warm-up")
        .build()
        .register()
});

pub static WORKOUT: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Workout")
        .icon('🎯')
        .description("Generic workout")
        .build()
        .register()
});

pub static TABATA: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Workout")
        .icon('🧨')
        .description("Generic workout")
        .build()
        .register()
});

pub static REST: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Rest")
        .icon('💤')
        .tags(bon::vec![Tag::Rest])
        .build()
        .register()
});

pub static WALK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Walk")
        .icon('🧍')
        .difficulty(Difficulty::Easy)
        .build()
        .register()
});

pub static RUN: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Run")
        .difficulty(Difficulty::Medium)
        .icon('🏃')
        .build()
        .register()
});

pub static SPRINT: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Sprint")
        .difficulty(Difficulty::Hard)
        .icon('🏃')
        .build()
        .register()
});

pub static SIDE_STEPS: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Side Steps")
        .icon('👯')
        .build()
        .register()
});

// PLANKS

pub static PLANK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Plank")
        .tags(bon::vec![Mouvement::Stationary, Body::Core, Body::Shoulder])
        .icon('🚪')
        .build()
        .register()
});

pub static PLANK_SHOULDER_TAP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Plank Shoulder Tap")
        .tags(bon::vec![Mouvement::Dynamic, Body::Core, Body::Shoulder])
        .icon('🚪')
        .build()
        .register()
});

pub static SIDE_PLANK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Side Plank")
        .tags(bon::vec![Mouvement::Stationary])
        .icon('🚪')
        .build()
        .register()
});

pub static COMMANDO_PLANK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Commando Plank")
        .tags(bon::vec![Mouvement::Dynamic])
        .icon('🚪')
        .build()
        .register()
});

// WARM UP

pub static HEAD_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Head Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .icon('⚙')
        .build()
        .register()
});

pub static SHOULDER_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Shoulder Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .icon('⚙')
        .build()
        .register()
});

pub static ARM_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Arms Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .icon('⚙')
        .build()
        .register()
});

pub static ELBOW_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Elbows Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .icon('⚙')
        .build()
        .register()
});

pub static WRIST_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Wrists Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .icon('⚙')
        .build()
        .register()
});

pub static HIP_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Hips Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .icon('⚙')
        .build()
        .register()
});

pub static KNEE_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Knees Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .icon('⚙')
        .build()
        .register()
});

pub static FEET_ROTATION: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Feet Rotation")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Rotation])
        .icon('⚙')
        .build()
        .register()
});

pub static HEEL_RAISES: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Heels Raises")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Stationary])
        .icon('⚙')
        .build()
        .register()
});

pub static INCHWORM: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Inchworm")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Stretching])
        .icon('⚙')
        .build()
        .register()
});

pub static LEG_SWINGS_FRONT: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Leg Swings Front")
        .tags(bon::vec![Tag::WarmUp])
        .icon('⚙')
        .build()
        .register()
});

pub static LEG_SWINGS_SIDE: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Leg Swings Side")
        .tags(bon::vec![Tag::WarmUp])
        .icon('⚙')
        .build()
        .register()
});

pub static SINGLE_LEG_TOUCH_TOES: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Single Leg Touch Toes")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Stretching])
        .icon('⚙')
        .build()
        .register()
});

pub static WINDMILL: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Windmill")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Stretching])
        .icon('⚙')
        .build()
        .register()
});

pub static BUTT_KICKS: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Butt Kicks")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
        .icon('⚙')
        .build()
        .register()
});

pub static HIGH_KNEES: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("High Knees")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
        .icon('⚙')
        .build()
        .register()
});

pub static JUMPING_JACK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Jumping Jack")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
        .icon('💓')
        .build()
        .register()
});

pub static MOUNTAIN_CLIMBER: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Mountain Climber")
        .tags(bon::vec![Tag::WarmUp, Mouvement::Dynamic])
        .icon('💓')
        .build()
        .register()
});

pub static SQUAT: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Squat")
        .tags(bon::vec![Mouvement::Dynamic])
        .icon('💓')
        .build()
        .register()
});

pub static SQUAT_BULGARIAN: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Squat Bulgarian")
        .tags(bon::vec![Mouvement::Dynamic])
        .icon('💓')
        .build()
        .register()
});

pub static SQUAT_SINGLE_LEG: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Squat Single Leg")
        .tags(bon::vec![Mouvement::Dynamic])
        .icon('💓')
        .build()
        .register()
});

pub static SQUAT_JUMP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Squat Jump")
        .tags(bon::vec![Mouvement::Dynamic])
        .icon('💓')
        .build()
        .register()
});

pub static SQUAT_SUMO: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Squat Sumo")
        .tags(bon::vec![Mouvement::Dynamic])
        .icon('💓')
        .build()
        .register()
});

pub static PUSH_UP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Push Up")
        .tags(bon::vec![Mouvement::Dynamic, Mouvement::Strength])
        .icon('💓')
        .build()
        .register()
});

pub static PULL_UP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Pull Up")
        .tags(bon::vec![Mouvement::Dynamic, Mouvement::Strength])
        .icon('💓')
        .build()
        .register()
});

pub static SPEED_STEP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Speed Step")
        .tags(bon::vec![Mouvement::Dynamic, Mouvement::Footwork])
        .icon('💓')
        .build()
        .register()
});

pub static ALTERNATE_STEP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Alternate Step")
        .tags(bon::vec![Mouvement::Dynamic, Mouvement::Footwork])
        .icon('💓')
        .build()
        .register()
});

pub static LUNGE: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Lunge")
        .tags(bon::vec![
            Mouvement::Dynamic,
            Mouvement::Balance,
            Mouvement::Strength
        ])
        .icon('💓')
        .build()
        .register()
});

pub static LUNGE_REVERSE: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Lunge Reverse")
        .icon('💓')
        .tags(bon::vec![
            Mouvement::Dynamic,
            Mouvement::Balance,
            Mouvement::Strength
        ])
        .build()
        .register()
});

pub static LUNGE_JUMP: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Lunge Jump")
        .icon('💓')
        .tags(bon::vec![
            Mouvement::Dynamic,
            Mouvement::Balance,
            Mouvement::Strength
        ])
        .difficulty(Difficulty::Medium)
        .build()
        .register()
});

pub static LUNGE_WALKING: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Lunge Walking")
        .tags(bon::vec![
            Mouvement::Dynamic,
            Mouvement::Balance,
            Mouvement::Strength
        ])
        .icon('💓')
        .build()
        .register()
});

pub static BURPEE: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Burpee")
        .tags(bon::vec![
            Mouvement::Dynamic,
            Body::Full,
            Mouvement::Stamina
        ])
        .icon('💓')
        .difficulty(Difficulty::Medium)
        .build()
        .register()
});

pub static BURPEE_NAVY_SEAL: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Burpee Navy Seal")
        .tags(bon::vec![
            Mouvement::Dynamic,
            Body::Full,
            Mouvement::Stamina
        ])
        .icon('💓')
        .build()
        .register()
});

pub static JUMPS: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Jumps")
        .tags(bon::vec![Mouvement::Dynamic, Mouvement::Stamina])
        .icon('💓')
        .build()
        .register()
});

pub static JUMPS_FORWARD: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Jumps Forward")
        .tags(bon::vec![
            Mouvement::Dynamic,
            Mouvement::Stamina,
            Mouvement::Balance
        ])
        .icon('💓')
        .build()
        .register()
});

pub static CRUNCHES: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Crunches")
        .tags(bon::vec![Body::Abs, Body::Core])
        .icon('💓')
        .build()
        .register()
});

pub static SCISSOR_KICK: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Scissor Kick")
        .tags(bon::vec![Body::Abs, Body::Hip, Body::Core])
        .icon('💓')
        .build()
        .register()
});

pub static HIP_THRUST: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Hip Thrust")
        .tags(bon::vec![Body::Hip, Body::Core, Body::Buttocks])
        .icon('💓')
        .build()
        .register()
});

pub static BOXING_ROUND: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Boxing Round")
        .icon('🥊')
        .tags(bon::vec![Tag::Boxing, Mouvement::Stamina, Body::Full])
        .build()
        .register()
});

pub static JUMP_ROPE: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Jump Rope")
        .tags(bon::vec![
            Tag::Boxing,
            Mouvement::Dynamic,
            Mouvement::Coordination,
            Equipment::JumpRope,
        ])
        .icon('🪱')
        .build()
        .register()
});

pub static JUMP_ROPE_DOUBLE_UNDERS: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Jump Rope Double Unders")
        .tags(bon::vec![
            Tag::Boxing,
            Mouvement::Dynamic,
            Mouvement::Coordination,
            Equipment::JumpRope,
        ])
        .icon('🪱')
        .difficulty(Difficulty::Medium)
        .build()
        .register()
});

pub static JUMP_ROPE_CRISS_CROSS: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Jump Rope Criss Cross")
        .tags(bon::vec![
            Tag::Boxing,
            Mouvement::Dynamic,
            Mouvement::Coordination,
            Equipment::JumpRope,
        ])
        .difficulty(Difficulty::Medium)
        .icon('🪱')
        .build()
        .register()
});

pub static JUMP_ROPE_HIGH_KNEES: std::sync::LazyLock<Item> = std::sync::LazyLock::new(|| {
    Item::builder()
        .name("Jump Rope High Knees")
        .tags(bon::vec![
            Tag::Boxing,
            Mouvement::Dynamic,
            Mouvement::Coordination,
            Equipment::JumpRope,
        ])
        .difficulty(Difficulty::Medium)
        .icon('🪱')
        .build()
        .register()
});
