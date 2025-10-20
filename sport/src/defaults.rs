use crate::exercises::Exercises;
use crate::item_list::ItemList;
use crate::sequence::{ROUNDS, Sequence};
use crate::sound::Sound;
use crate::tag::{Body, Difficulty, Tag};
use itertools::Itertools;
use natural_sort_rs::Natural;
use strum::VariantArray;
use time::ext::NumericalStdDuration;

pub const DEFAULT_ICON: char = '🎯';
pub const DEFAULT_INTERVAL: u32 = 1000;
pub const PREPARE: u64 = 15;
pub const PREPARE_DURATION: std::time::Duration = std::time::Duration::from_secs(PREPARE);
// pub const PREPARE_LABEL: &str = "⏳";
pub const PREPARE_LABEL: &str = "Prepare";
// pub const RESTART_SEQUENCE: &str = "♼";
pub const RESTART_SEQUENCE: &str = "♻";
pub const PREVIOUS_ITEM: &str = "⏪";
pub const NEXT_ITEM: &str = "⏩";
pub const RANDOMIZE: &str = "🎲";
pub const SIGNAL: &str = "🛎";

pub static SEQUENCES: std::sync::LazyLock<Vec<Sequence>> = std::sync::LazyLock::new(|| {
    let mut sequences = Vec::new();

    let warmup = Sequence::simple()
        .name("Warm-up (articular)")
        .workouts(&[
            // 1 minute
            ItemList::HeadRotation.easy(20.std_seconds()),
            ItemList::ShoulderRotation.easy(20.std_seconds()),
            ItemList::ArmRotation.easy(20.std_seconds()),
            // 1 minute
            ItemList::ElbowRotation.easy(20.std_seconds()),
            ItemList::WristRotation.easy(20.std_seconds()),
            ItemList::HipRotation.easy(20.std_seconds()),
            // 1 minute
            ItemList::KneeRotation.easy(20.std_seconds()),
            ItemList::AnkleRotation.easy(20.std_seconds()),
            ItemList::HeelRaise.easy(20.std_seconds()),
            // 1 minute
            ItemList::LegSwingFront.easy(20.std_seconds()),
            ItemList::LegSwingSide.easy(20.std_seconds()),
            ItemList::LegTouchToe.easy(20.std_seconds()),
            // 1 minute
            ItemList::ButtKicks.easy(30.std_seconds()),
            ItemList::HighKnees.easy(30.std_seconds()),
            // 1 minute
            ItemList::JumpingJack.easy(30.std_seconds()),
            ItemList::MountainClimber.easy(30.std_seconds()),
            // 1 minute
            ItemList::Squat.medium(30.std_seconds()),
            ItemList::PushUp.medium(30.std_seconds()),
            // 1 minute
            ItemList::SpeedStep.easy(30.std_seconds()),
            ItemList::SkatingStep.easy(30.std_seconds()),
            // 1 minute
            ItemList::Lunge.medium(30.std_seconds()),
            ItemList::Burpee.medium(30.std_seconds()),
        ])
        .difficulty(Difficulty::Easy)
        .sound(&Sound::Silent)
        .icon('🔥')
        .call();
    sequences.push(warmup);

    let cardio_warmup = Sequence::simple()
        .name("Warm-up (cardio)")
        .workouts(&[
            // 1 minute
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::HighKnees.easy(15.std_seconds()),
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::ButtKicks.easy(15.std_seconds()),
            // 1 minute
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::SkatingStep.easy(15.std_seconds()),
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::MountainClimber.easy(15.std_seconds()),
            // 1 minute
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::Squat.easy(15.std_seconds()),
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::Lunge.easy(15.std_seconds()),
            // 1 minute
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::SquatJump.easy(15.std_seconds()),
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::LungeJumping.medium(15.std_seconds()),
            // 1 minute
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::Jump.easy(15.std_seconds()),
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::PushUp.easy(15.std_seconds()),
            // 1 minute
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::CommandoPlank.easy(15.std_seconds()),
            ItemList::JumpingJack.easy(15.std_seconds()),
            ItemList::Burpee.easy(15.std_seconds()),
        ])
        .difficulty(Difficulty::Easy)
        .icon('💓')
        .sound(&Sound::Beep)
        .call();
    sequences.push(cardio_warmup);

    for x in [5, 10, 15] {
        let rounds = Sequence::rounds()
            .name(&format!("1m/{x}x Workout"))
            .rounds(x)
            .workout(ItemList::Workout.workout(1.std_minutes()))
            .rest(30.std_seconds())
            .sound(&Sound::Beep)
            .difficulty(Difficulty::Medium)
            .icon('🥊')
            .call();
        sequences.push(rounds);
    }

    for x in [2, 3, 4, 6, 12] {
        let rounds = Sequence::rounds()
            .name(&format!("{x}x2m"))
            .rounds(x)
            .workout(ItemList::BoxingRound.workout(2.std_minutes()))
            .rest(30.std_seconds())
            .sound(&Sound::Bell)
            .icon('🥊')
            .difficulty(Difficulty::Medium)
            .call();
        sequences.push(rounds);
    }

    for x in [2, 3, 4, 6, 12] {
        let rounds = Sequence::rounds()
            .name(&format!("{x}x3m"))
            .rounds(x)
            .workout(ItemList::BoxingRound.workout(3.std_minutes()))
            .rest(1.std_minutes())
            .sound(&Sound::Bell)
            .icon('🥊')
            .difficulty(Difficulty::Hard)
            .call();
        sequences.push(rounds);
    }

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
        .workout(30.std_seconds())
        .icon('🥊')
        .rounds(4 * ROUNDS)
        .rest(1.std_minutes())
        .difficulty(Difficulty::Medium)
        .sound(&Sound::Bell)
        .call();
    sequences.push(stamina_jab_cross_hook);

    let stamina_uppercuts_hook = Sequence::repeat()
        .name("Stamina 30s 5-6-3-2")
        .description("5 | 5-6 | 5-6-3 | 5-6-3-2")
        .exercises(Exercises::from_strings(
            vec![
                "Lead Uppercut (5)",
                "Lead Uppercut | Rear Uppercut (5-6)",
                "Lead Uppercut | Rear Uppercut | Hook (5-6-3)",
                "Lead Uppercut | Rear Uppercut | Hook | Cross (5-6-3-2)",
            ],
            Some('🥊'),
        ))
        .workout(30.std_seconds())
        .icon('🥊')
        .rounds(4 * ROUNDS)
        .rest(1.std_minutes())
        .difficulty(Difficulty::Medium)
        .sound(&Sound::Bell)
        .call();
    sequences.push(stamina_uppercuts_hook);

    let stamina_jab_cross_uppercut_cross = Sequence::repeat()
        .name("Stamina 30s 1-2-5-2")
        .description("1 | 1-2 | 1-2-5 | 1-2-5-2")
        .exercises(Exercises::from_strings(
            vec![
                "Jab (1)",
                "Jab | Cross (1-2)",
                "Jab | Cross | Uppercut (1-2-5)",
                "Jab | Cross | Uppercut | Cross (1-2-5-2)",
            ],
            Some('🥊'),
        ))
        .workout(30.std_seconds())
        .icon('🥊')
        .rounds(4 * ROUNDS)
        .rest(1.std_minutes())
        .difficulty(Difficulty::Medium)
        .sound(&Sound::Bell)
        .call();
    sequences.push(stamina_jab_cross_uppercut_cross);

    let stamina_jab_jab_cross_cross = Sequence::repeat()
        .name("Stamina 30s 1-1-2-2")
        .description("1 | 1-1 | 1-1-2 | 1-1-2-2")
        .exercises(Exercises::from_strings(
            vec![
                "Jab (1)",
                "Jab | Jab (1-1)",
                "Jab | Jab | Cross (1-1-2)",
                "Jab | Jab | Cross | Cross (1-1-2-2)",
            ],
            Some('🥊'),
        ))
        .workout(30.std_seconds())
        .icon('🥊')
        .rounds(4 * ROUNDS)
        .rest(1.std_minutes())
        .difficulty(Difficulty::Medium)
        .sound(&Sound::Bell)
        .call();
    sequences.push(stamina_jab_jab_cross_cross);

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
        .workout(30.std_seconds())
        .rounds(4 * ROUNDS)
        .icon('🥊')
        .rest(1.std_minutes())
        .difficulty(Difficulty::Medium)
        .sound(&Sound::Bell)
        .call();
    sequences.push(stamina_jab_cross_hook_cross);

    let stamina_roll_left = Sequence::repeat()
        .name("Stamina 30s 1-2-3-ROLL-3-2")
        .description("1-2-3 | 1-2-3-ROLL | 1-2-3-ROLL-3 | 1-2-3-ROLL-3-2")
        .exercises(Exercises::from_strings(
            vec![
                "Jab | Cross | Hook (1-2-3)",
                "Jab | Cross | Hook | ROLL (1-2-3-R)",
                "Jab | Cross | Hook | ROLL | Hook (1-2-3-R-3)",
                "Jab | Cross | Hook | ROLL | Hook | Cross (1-2-3-R-3-2)",
            ],
            Some('🥊'),
        ))
        .workout(30.std_seconds())
        .icon('🥊')
        .rounds(4 * ROUNDS)
        .rest(1.std_minutes())
        .difficulty(Difficulty::Hard)
        .sound(&Sound::Bell)
        .call();
    sequences.push(stamina_roll_left);

    let stamina_roll_right = Sequence::repeat()
        .name("Stamina 30s 1-2-ROLL-2-3-2")
        .description("1-2-ROLL | 1-2-ROLL-2 | 1-2-ROLL-2-3 | 1-2-ROLL-2-3-2")
        .exercises(Exercises::from_strings(
            vec![
                "Jab | Cross | ROLL (1-2-R)",
                "Jab | Cross | ROLL | Cross (1-2-R-2)",
                "Jab | Cross | ROLL | Cross | Hook (1-2-R-2-3)",
                "Jab | Cross | ROLL | Cross | Hook | Cross (1-2-R-2-3-2)",
            ],
            Some('🥊'),
        ))
        .workout(30.std_seconds())
        .rounds(4 * ROUNDS)
        .rest(1.std_minutes())
        .icon('🥊')
        .difficulty(Difficulty::Hard)
        .sound(&Sound::Bell)
        .call();
    sequences.push(stamina_roll_right);

    let _2_boxing_combinations = Sequence::repeat()
        .name("Boxing 2 combos")
        .description("All realistic boxing combinations")
        .exercises(Exercises::from_strings(
            vec![
                "Jab | Jab (1-1)",
                "Jab | Cross (1-2)",
                "Cross | Hook (2-3)",
                "Lead Uppercut | Rear Uppercut (5-6)",
                "Lead Uppercut | Cross (5-2)",
                "Jab | Hook (1-4)",
                "Rear Uppercut | Hook (6-3)",
                "Lead Hook | Rear Hook (3-4)",
            ],
            Some('🥊'),
        ))
        .workout(30.std_seconds())
        .rounds(4 * ROUNDS)
        .rest(1.std_minutes())
        .icon('🥊')
        .difficulty(Difficulty::Hard)
        .sound(&Sound::Bell)
        .call();
    sequences.push(_2_boxing_combinations);

    let _3_boxing_combinations = Sequence::repeat()
        .name("Boxing 3 combos")
        .description("All realistic boxing combinations")
        .exercises(Exercises::from_strings(
            vec![
                "Jab | Jab | Jab (1-1-1)",
                "Jab | Jab | Cross (1-1-2)",
                "Jab | Cross | (Pull) | Cross (1-2-P-2)",
                "Jab | Cross | (Slip) | Rear Uppercut (1-2-S-6)",
                "Jab | Cross | Jab (1-2-1)",
                "Jab | Cross | Hook (1-2-3)",
                "Cross | Hook | Cross (2-3-2)",
                "Jab | Cross | Lead Uppercut (1-2-5)",
                "Lead Uppercut | Cross | Hook (5-2-3)",
                "Lead Uppercut | Rear Uppercut | Hook (5-6-3)",
                "Cross | Rear Uppercut | Hook (2-6-3)",
            ],
            Some('🥊'),
        ))
        .workout(30.std_seconds())
        .rounds(4 * ROUNDS)
        .rest(1.std_minutes())
        .icon('🥊')
        .difficulty(Difficulty::Hard)
        .sound(&Sound::Bell)
        .call();
    sequences.push(_3_boxing_combinations);

    for x in [4, 8] {
        let hiit = Sequence::rounds()
            .name(&format!("HiiT {x}x20s"))
            .rounds(x)
            .workout(ItemList::Tabata.workout(20.std_seconds()))
            .difficulty(Difficulty::Medium)
            .icon('🧨')
            .rest(10.std_seconds())
            .sound(&Sound::Beep)
            .call();
        sequences.push(hiit);
    }

    let _4x_hiit_8x = Sequence::rounds()
        .name("HiiT 8x20s (4x)")
        .rounds(8 * ROUNDS)
        .workout(ItemList::Tabata.workout(20.std_seconds()))
        .difficulty(Difficulty::Hard)
        .icon('🧨')
        .rest(10.std_seconds())
        .sound(&Sound::Beep)
        .call()
        .cycle(4, 1.std_minutes());

    for x in [1, 2, 3, 5, 10, 15] {
        let seq = ItemList::Workout
            .workout(x.std_minutes())
            .sequence(&Sound::Bell);
        sequences.push(seq);
    }

    // let mut random_tags = vec![Tag::Boxing];
    let mut random_tags = vec![];
    let mut body_tags = Body::VARIANTS.iter().map(|v| Tag::Body(*v)).collect_vec();
    random_tags.append(&mut body_tags);

    // info!("{random_tags:?}");

    let random_workouts = Exercises::from_tags(&random_tags).workouts(30.std_seconds());
    let random_training = Sequence::random()
        .name("Random training (30s)")
        .workouts(random_workouts)
        .icon('🎲')
        .rest(10.std_seconds())
        .sound(&Sound::Beep)
        .call();
    sequences.push(random_training);

    sequences.sort_by_key(|s| Natural::str(s.name().to_string()));
    sequences
});
