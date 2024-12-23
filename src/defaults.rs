use crate::duration::{MINUTE, SECOND};
use crate::exercises::Exercises;
use crate::item;
use crate::sequence::{Sequence, ROUNDS};
use crate::signal::SoundSignal;
use crate::tag::Difficulty;

pub const DEFAULT_INTERVAL: u32 = 1000;
pub const PREPARE: u64 = 15;
pub const PREPARE_DURATION: std::time::Duration = std::time::Duration::from_secs(PREPARE);
pub const PREPARE_LABEL: &str = "⏳";
pub const RESTART_SEQUENCE: &str = "♼";
pub const PREVIOUS_ITEM: &str = "⏪";
pub const NEXT_ITEM: &str = "⏩";
pub const RANDOMIZE: &str = "🎲";
pub const SIGNAL: &str = "🛎";

pub fn default_sequences(
    bell: &SoundSignal,
    beep: &SoundSignal,
    silent: &SoundSignal,
) -> Vec<Sequence> {
    let warmup = Sequence::simple()
        .name("🔥Warm-up")
        .workouts(&[
            // 1 minute
            item::HEAD_ROTATION.easy(20 * SECOND),
            item::SHOULDER_ROTATION.easy(20 * SECOND),
            item::ARM_ROTATION.easy(20 * SECOND),
            // 1 minute
            item::ELBOW_ROTATION.easy(20 * SECOND),
            item::WRIST_ROTATION.easy(20 * SECOND),
            item::HIP_ROTATION.easy(20 * SECOND),
            // 1 minute
            item::KNEE_ROTATION.easy(20 * SECOND),
            item::FEET_ROTATION.easy(20 * SECOND),
            item::HEEL_RAISES.easy(20 * SECOND),
            // 1 minute
            item::LEG_SWINGS.easy(20 * SECOND),
            item::SIDE_LEG_SWINGS.easy(20 * SECOND),
            item::SINGLE_LEG_TOUCH_TOES.easy(20 * SECOND),
            // 1 minute
            item::BUTT_KICKS.easy(30 * SECOND),
            item::HIGH_KNEES.easy(30 * SECOND),
            // 1 minute
            item::JUMPING_JACK.easy(30 * SECOND),
            item::MOUNTAIN_CLIMBER.easy(30 * SECOND),
            // 1 minute
            item::SQUAT.medium(30 * SECOND),
            item::PUSH_UP.medium(30 * SECOND),
            // 1 minute
            item::SPEED_STEP.easy(30 * SECOND),
            item::ALTERNATE_STEP.easy(30 * SECOND),
            // 1 minute
            item::LUNGE.medium(30 * SECOND),
            item::BURPEE.medium(30 * SECOND),
        ])
        .signal(silent)
        .call();

    let cardio_warmup = Sequence::simple()
        .name("💓Warm-up")
        .workouts(&[
            // 1 minute
            item::JUMPING_JACK.easy(15 * SECOND),
            item::HIGH_KNEES.easy(15 * SECOND),
            item::JUMPING_JACK.easy(15 * SECOND),
            item::BUTT_KICKS.easy(15 * SECOND),
            // 1 minute
            item::JUMPING_JACK.easy(15 * SECOND),
            item::ALTERNATE_STEP.easy(15 * SECOND),
            item::JUMPING_JACK.easy(15 * SECOND),
            item::MOUNTAIN_CLIMBER.easy(15 * SECOND),
            // 1 minute
            item::JUMPING_JACK.easy(15 * SECOND),
            item::SQUAT.easy(15 * SECOND),
            item::JUMPING_JACK.easy(15 * SECOND),
            item::LUNGE.easy(15 * SECOND),
            // 1 minute
            item::JUMPING_JACK.easy(15 * SECOND),
            item::JUMP_SQUAT.easy(15 * SECOND),
            item::JUMPING_JACK.easy(15 * SECOND),
            item::LUNGE.medium(15 * SECOND),
            // 1 minute
            item::JUMPING_JACK.easy(15 * SECOND),
            item::JUMPS.easy(15 * SECOND),
            item::JUMPING_JACK.easy(15 * SECOND),
            item::PUSH_UP.easy(15 * SECOND),
            // 1 minute
            item::JUMPING_JACK.easy(15 * SECOND),
            item::COMMANDO_PLANK.easy(15 * SECOND),
            item::JUMPING_JACK.easy(15 * SECOND),
            item::BURPEE.easy(15 * SECOND),
        ])
        .signal(beep)
        .call();

    let _5_rounds_1m = Sequence::rounds()
        .name("🥊1m/5x Workout")
        .rounds(5)
        .workout(item::WORKOUT.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .signal(beep)
        .difficulty(Difficulty::Medium)
        .call();

    let _10_rounds_1m = Sequence::rounds()
        .name("🥊1m/10x Workout")
        .rounds(10)
        .workout(item::WORKOUT.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .signal(beep)
        .difficulty(Difficulty::Medium)
        .call();

    let _15_rounds_1m = Sequence::rounds()
        .name("🥊1m/15x Workout")
        .rounds(10)
        .workout(item::WORKOUT.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .signal(beep)
        .difficulty(Difficulty::Medium)
        .call();

    let boxing_3x2m_30s = Sequence::rounds()
        .name("🥊3x2m")
        .rounds(3 * ROUNDS)
        .workout(item::BOXING_ROUND.workout(2 * MINUTE))
        .rest(30 * SECOND)
        .signal(bell)
        .difficulty(Difficulty::Easy)
        .call();

    let boxing_3x3m_1m = Sequence::rounds()
        .name("🥊3x3m")
        .rounds(3 * ROUNDS)
        .workout(item::BOXING_ROUND.workout(2 * MINUTE))
        .rest(MINUTE)
        .signal(bell)
        .difficulty(Difficulty::Easy)
        .call();

    let boxing_6x2m_30s = Sequence::rounds()
        .name("🥊6x2m")
        .rounds(6 * ROUNDS)
        .workout(item::BOXING_ROUND.workout(2 * MINUTE))
        .rest(30 * SECOND)
        .signal(bell)
        .difficulty(Difficulty::Medium)
        .call();

    let boxing_6x3m_1m = Sequence::rounds()
        .name("🥊6x3m")
        .rounds(6 * ROUNDS)
        .workout(item::BOXING_ROUND.workout(3 * MINUTE))
        .rest(MINUTE)
        .signal(bell)
        .difficulty(Difficulty::Hard)
        .call();

    let stamina_jab_cross_hook = Sequence::repeat()
        .name("🥊Stamina 30s 1-2-3")
        .description("1 | 2 | 1-2 | 1-2-3")
        .exercises(Exercises::from(vec![
            "🥊Jab (1)",
            "🥊Cross (2)",
            "🥊Jab | Cross (1-2)",
            "🥊Jab | Cross | Hook (1-2-3)",
        ]))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Medium)
        .signal(bell)
        .call();

    let stamina_jab_jab_cross_cross = Sequence::repeat()
        .name("🥊Stamina 30s 1-1-2-2")
        .description("1 | 1-1 | 1-1-2 | 1-1-2-2")
        .exercises(Exercises::from(vec![
            "🥊Jab (1)",
            "🥊Double Jab (1-1)",
            "🥊Double Jab | Cross (1-1-2)",
            "🥊Double Jab | Cross | Cross",
        ]))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Medium)
        .signal(bell)
        .call();

    let stamina_jab_cross_hook_cross = Sequence::repeat()
        .name("🥊Stamina 30s 1-2-3-2")
        .description("1 | 1-2 | 1-2-3 | 1-2-3-2")
        .exercises(Exercises::from(vec![
            "🥊Jab (1)",
            "🥊Jab | Cross (1-2)",
            "🥊Jab | Cross | Hook (1-2-3)",
            "🥊Jab | Cross | Hook | Cross (1-2-3-2)",
        ]))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Medium)
        .signal(bell)
        .call();

    let stamina_roll_left = Sequence::repeat()
        .name("🥊Stamina 30s 1-2-3-ROLL-3-2")
        .description("1-2-3 | 1-2-3-ROLL | 1-2-3-ROLL-3 | 1-2-3-ROLL-3-2")
        .exercises(Exercises::from(vec![
            "🥊Jab | Cross | Hook",
            "🥊Jab | Cross | Hook | ROLL",
            "🥊Jab | Cross | Hook | ROLL | Hook",
            "🥊Jab | Cross | Hook | ROLL | Hook | Cross",
        ]))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Hard)
        .signal(bell)
        .call();

    let stamina_roll_right = Sequence::repeat()
        .name("🥊Stamina 30s 1-2-ROLL-2-3-2")
        .description("1-2-ROLL | 1-2-ROLL-2 | 1-2-ROLL-2-3 | 1-2-ROLL-2-3-2")
        .exercises(Exercises::from(vec![
            "🥊Jab | Cross | ROLL",
            "🥊Jab | Cross | ROLL | Cross",
            "🥊Jab | Cross | ROLL | Cross | Hook",
            "🥊Jab | Cross | ROLL | Cross | Hook | Cross",
        ]))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Hard)
        .signal(bell)
        .call();

    let random_training = Sequence::random()
        .name("🎲Random training (30s)")
        .workouts(&[
            item::JUMPING_JACK.workout(30 * SECOND),
            item::PULL_UP.workout(30 * SECOND),
            item::PLANK.workout(30 * SECOND),
            item::SIDE_PLANK.workout(30 * SECOND),
            item::JUMP_SQUAT.workout(30 * SECOND),
            item::BURPEE.workout(30 * SECOND),
            item::PUSH_UP.workout(30 * SECOND),
            item::LUNGE.workout(30 * SECOND),
            item::CRUNCHES.workout(30 * SECOND),
        ])
        .rest(30 * SECOND)
        .signal(beep)
        .call();

    let hiit_4x = Sequence::rounds()
        .name("🧨HiiT 20s (4x)")
        .rounds(4 * ROUNDS)
        .workout(item::WORKOUT.workout(20 * SECOND))
        .difficulty(Difficulty::Medium)
        .rest(10 * SECOND)
        .signal(beep)
        .call();

    let hiit_8x = Sequence::rounds()
        .name("🧨HiiT 20s (8x)")
        .rounds(8 * ROUNDS)
        .workout(item::WORKOUT.workout(20 * SECOND))
        .difficulty(Difficulty::Hard)
        .rest(10 * SECOND)
        .signal(beep)
        .call();

    let _1mn = item::WORKOUT.workout(1 * MINUTE).sequence(bell);
    let _2mn = item::WORKOUT.workout(2 * MINUTE).sequence(bell);
    let _3mn = item::WORKOUT.workout(3 * MINUTE).sequence(bell);
    let _5mn = item::WORKOUT.workout(5 * MINUTE).sequence(bell);
    let _10mn = item::WORKOUT.workout(10 * MINUTE).sequence(bell);
    let _15mn = item::WORKOUT.workout(15 * MINUTE).sequence(bell);

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
