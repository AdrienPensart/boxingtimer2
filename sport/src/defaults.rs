use crate::duration::{MINUTE, SECOND};
use crate::exercises::Exercises;
use crate::item_list::ItemList;
use crate::sequence::{Sequence, ROUNDS};
use crate::sound::Sound;
use crate::tag::{Body, Difficulty, Tag};
use itertools::Itertools;
use strum::VariantArray;

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

pub static SEQUENCES: std::sync::LazyLock<Vec<Sequence>> = std::sync::LazyLock::new(|| {
    let warmup = Sequence::simple()
        .name("Warm-up (articular)")
        .workouts(&[
            // 1 minute
            ItemList::HeadRotation.easy(20 * SECOND),
            ItemList::ShoulderRotation.easy(20 * SECOND),
            ItemList::ArmRotation.easy(20 * SECOND),
            // 1 minute
            ItemList::ElbowRotation.easy(20 * SECOND),
            ItemList::WristRotation.easy(20 * SECOND),
            ItemList::HipRotation.easy(20 * SECOND),
            // 1 minute
            ItemList::KneeRotation.easy(20 * SECOND),
            ItemList::AnkleRotation.easy(20 * SECOND),
            ItemList::HeelRaise.easy(20 * SECOND),
            // 1 minute
            ItemList::LegSwingFront.easy(20 * SECOND),
            ItemList::LegSwingSide.easy(20 * SECOND),
            ItemList::LegTouchToe.easy(20 * SECOND),
            // 1 minute
            ItemList::ButtKicks.easy(30 * SECOND),
            ItemList::HighKnees.easy(30 * SECOND),
            // 1 minute
            ItemList::JumpingJack.easy(30 * SECOND),
            ItemList::MountainClimber.easy(30 * SECOND),
            // 1 minute
            ItemList::Squat.medium(30 * SECOND),
            ItemList::PushUp.medium(30 * SECOND),
            // 1 minute
            ItemList::SpeedStep.easy(30 * SECOND),
            ItemList::SkatingStep.easy(30 * SECOND),
            // 1 minute
            ItemList::Lunge.medium(30 * SECOND),
            ItemList::Burpee.medium(30 * SECOND),
        ])
        .difficulty(Difficulty::Easy)
        .sound(&Sound::Silent)
        .icon('🔥')
        .call();

    let cardio_warmup = Sequence::simple()
        .name("Warm-up (cardio)")
        .workouts(&[
            // 1 minute
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::HighKnees.easy(15 * SECOND),
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::ButtKicks.easy(15 * SECOND),
            // 1 minute
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::SkatingStep.easy(15 * SECOND),
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::MountainClimber.easy(15 * SECOND),
            // 1 minute
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::Squat.easy(15 * SECOND),
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::Lunge.easy(15 * SECOND),
            // 1 minute
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::SquatJump.easy(15 * SECOND),
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::LungeJumping.medium(15 * SECOND),
            // 1 minute
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::Jump.easy(15 * SECOND),
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::PushUp.easy(15 * SECOND),
            // 1 minute
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::CommandoPlank.easy(15 * SECOND),
            ItemList::JumpingJack.easy(15 * SECOND),
            ItemList::Burpee.easy(15 * SECOND),
        ])
        .difficulty(Difficulty::Easy)
        .icon('💓')
        .sound(&Sound::Beep)
        .call();

    let _5_rounds_1m = Sequence::rounds()
        .name("1m/5x Workout")
        .rounds(5)
        .workout(ItemList::Workout.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .sound(&Sound::Beep)
        .difficulty(Difficulty::Medium)
        .icon('🥊')
        .call();

    let _10_rounds_1m = Sequence::rounds()
        .name("1m/10x Workout")
        .rounds(10)
        .workout(ItemList::Workout.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .sound(&Sound::Beep)
        .difficulty(Difficulty::Medium)
        .icon('🥊')
        .call();

    let _15_rounds_1m = Sequence::rounds()
        .name("1m/15x Workout")
        .rounds(10)
        .workout(ItemList::Workout.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .sound(&Sound::Beep)
        .icon('🥊')
        .difficulty(Difficulty::Medium)
        .call();

    let boxing_3x2m_30s = Sequence::rounds()
        .name("3x2m")
        .rounds(3 * ROUNDS)
        .workout(ItemList::BoxingRound.workout(2 * MINUTE))
        .rest(30 * SECOND)
        .sound(&Sound::Bell)
        .icon('🥊')
        .difficulty(Difficulty::Easy)
        .call();

    let boxing_3x3m_1m = Sequence::rounds()
        .name("3x3m")
        .rounds(3 * ROUNDS)
        .workout(ItemList::BoxingRound.workout(2 * MINUTE))
        .rest(MINUTE)
        .sound(&Sound::Bell)
        .icon('🥊')
        .difficulty(Difficulty::Easy)
        .call();

    let boxing_6x2m_30s = Sequence::rounds()
        .name("6x2m")
        .rounds(6 * ROUNDS)
        .workout(ItemList::BoxingRound.workout(2 * MINUTE))
        .rest(30 * SECOND)
        .sound(&Sound::Bell)
        .icon('🥊')
        .difficulty(Difficulty::Medium)
        .call();

    let boxing_6x3m_1m = Sequence::rounds()
        .name("6x3m")
        .rounds(6 * ROUNDS)
        .workout(ItemList::BoxingRound.workout(3 * MINUTE))
        .rest(MINUTE)
        .sound(&Sound::Bell)
        .icon('🥊')
        .difficulty(Difficulty::Hard)
        .call();

    let boxing_12x3m_1m = Sequence::rounds()
        .name("12x3m")
        .rounds(12 * ROUNDS)
        .workout(ItemList::BoxingRound.workout(3 * MINUTE))
        .rest(MINUTE)
        .sound(&Sound::Bell)
        .icon('🥊')
        .difficulty(Difficulty::Elite)
        .call();

    let stamina_jab_cross_hook = Sequence::repeat()
        .name("Stamina 30s 1-2-3")
        .description("1 | 2 | 1-2 | 1-2-3")
        .exercises(Exercises::from_strings(
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
        .call();

    let stamina_jab_jab_cross_cross = Sequence::repeat()
        .name("Stamina 30s 1-1-2-2")
        .description("1 | 1-1 | 1-1-2 | 1-1-2-2")
        .exercises(Exercises::from_strings(
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
        .call();

    let stamina_jab_cross_hook_cross = Sequence::repeat()
        .name("Stamina 30s 1-2-3-2")
        .description("1 | 1-2 | 1-2-3 | 1-2-3-2")
        .exercises(Exercises::from_strings(
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
        .call();

    let stamina_roll_left = Sequence::repeat()
        .name("Stamina 30s 1-2-3-ROLL-3-2")
        .description("1-2-3 | 1-2-3-ROLL | 1-2-3-ROLL-3 | 1-2-3-ROLL-3-2")
        .exercises(Exercises::from_strings(
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
        .call();

    let stamina_roll_right = Sequence::repeat()
        .name("Stamina 30s 1-2-ROLL-2-3-2")
        .description("1-2-ROLL | 1-2-ROLL-2 | 1-2-ROLL-2-3 | 1-2-ROLL-2-3-2")
        .exercises(Exercises::from_strings(
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
        .call();

    let hiit_4x = Sequence::rounds()
        .name("Half-Tabata 20s (4x)")
        .rounds(4 * ROUNDS)
        .workout(ItemList::Tabata.workout(20 * SECOND))
        .difficulty(Difficulty::Medium)
        .icon('🧨')
        .rest(10 * SECOND)
        .sound(&Sound::Beep)
        .call();

    let hiit_8x = Sequence::rounds()
        .name("Tabata 20s (8x)")
        .rounds(8 * ROUNDS)
        .workout(ItemList::Tabata.workout(20 * SECOND))
        .difficulty(Difficulty::Hard)
        .icon('🧨')
        .rest(10 * SECOND)
        .sound(&Sound::Beep)
        .call();

    let _1mn = ItemList::Workout.workout(1 * MINUTE).sequence(&Sound::Bell);
    let _2mn = ItemList::Workout.workout(2 * MINUTE).sequence(&Sound::Bell);
    let _3mn = ItemList::Workout.workout(3 * MINUTE).sequence(&Sound::Bell);
    let _5mn = ItemList::Workout.workout(5 * MINUTE).sequence(&Sound::Bell);
    let _10mn = ItemList::Workout
        .workout(10 * MINUTE)
        .sequence(&Sound::Bell);
    let _15mn = ItemList::Workout
        .workout(15 * MINUTE)
        .sequence(&Sound::Bell);

    // let mut random_tags = vec![Tag::Boxing];
    let mut random_tags = vec![];
    let mut body_tags = Body::VARIANTS.iter().map(|v| Tag::Body(*v)).collect_vec();
    random_tags.append(&mut body_tags);

    // info!("{random_tags:?}");

    let random_workouts = Exercises::from_tags(&random_tags).workouts(30 * SECOND);
    let random_training = Sequence::random()
        .name("Random training (30s)")
        .workouts(random_workouts)
        .icon('🎲')
        .rest(10 * SECOND)
        .sound(&Sound::Beep)
        .call();

    let mut sequences = vec![
        random_training,
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
        _1mn,
        _2mn,
        _3mn,
        _5mn,
        _10mn,
        _15mn,
    ];
    sequences.sort_by_key(|s| s.name().to_string());
    sequences
});
