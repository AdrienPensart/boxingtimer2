#![allow(non_snake_case)]
pub mod duration;
pub mod errors;
pub mod indexedvec;
pub mod item;
pub mod sequence;
pub mod signal;
pub mod sound;
pub mod status;
pub mod stopwatch;
pub mod tag;
pub mod timer;
pub mod workout;
use crate::duration::DurationExt;
use crate::duration::{MINUTE, SECOND};
use crate::sequence::{Exercises, Sequence, ROUNDS};
use crate::signal::{Signal, State};
use crate::sound::Sound;
use crate::tag::Difficulty;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use document::Stylesheet;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/?:muted&:prepare&:sequence")]
    BoxingTimer {
        muted: bool,
        prepare: u64,
        sequence: String,
    },
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(App);
}

fn App() -> Element {
    rsx! {
        Stylesheet { href: asset!("/assets/tailwind.css") }
        Router::<Route> {}
    }
}

#[component]
fn BoxingTimer(muted: bool, prepare: u64, sequence: String) -> Element {
    let state = if muted {
        State::Disabled
    } else {
        State::Enabled
    };
    if prepare == 0 {
        prepare = 20;
    }
    let state = Rc::new(RefCell::new(state));

    let silent = Signal::new(Sound::Silent, state.clone());
    let bell = Signal::new(Sound::Bell, state.clone());
    let beep = Signal::new(Sound::Beep, state.clone());

    let preparation = std::time::Duration::from_secs(prepare);

    let warmup = Sequence::simple()
        .name("Warm-up : 🌡")
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
            item::JUMP_SQUAT.medium(30 * SECOND),
            item::PUSH_UP.medium(30 * SECOND),
            // 1 minute
            item::SPEED_STEP.easy(30 * SECOND),
            item::ALTERNATE_STEP.easy(30 * SECOND),
            // 1 minute
            item::ALTERNATE_LUNGE.medium(30 * SECOND),
            item::BURPEE.medium(30 * SECOND),
        ])
        .signal(&silent)
        .call();

    let cardio_warmup = Sequence::simple()
        .name("Cardio Warm-up : ❤️‍🔥")
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
            item::ALTERNATE_LUNGE.medium(15 * SECOND),
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
        .signal(&beep)
        .call();

    let _5_rounds_1m = Sequence::rounds()
        .name("1m/5x Workout")
        .rounds(5)
        .workout(item::WORKOUT.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .signal(&beep)
        .difficulty(Difficulty::Medium)
        .call();

    let _10_rounds_1m = Sequence::rounds()
        .name("1m/10x Workout")
        .rounds(10)
        .workout(item::WORKOUT.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .signal(&beep)
        .difficulty(Difficulty::Medium)
        .call();

    let _15_rounds_1m = Sequence::rounds()
        .name("1m/15x Workout")
        .rounds(10)
        .workout(item::WORKOUT.workout(1 * MINUTE))
        .rest(30 * SECOND)
        .signal(&beep)
        .difficulty(Difficulty::Medium)
        .call();

    let boxing_3x2m_30s = Sequence::rounds()
        .name("3x2m")
        .rounds(3 * ROUNDS)
        .workout(item::BOXING_ROUND.workout(2 * MINUTE))
        .rest(30 * SECOND)
        .signal(&bell)
        .difficulty(Difficulty::Easy)
        .call();

    let boxing_3x3m_1m = Sequence::rounds()
        .name("3x3m")
        .rounds(3 * ROUNDS)
        .workout(item::BOXING_ROUND.workout(2 * MINUTE))
        .rest(MINUTE)
        .signal(&bell)
        .difficulty(Difficulty::Easy)
        .call();

    let boxing_6x2m_30s = Sequence::rounds()
        .name("6x2m")
        .rounds(6 * ROUNDS)
        .workout(item::BOXING_ROUND.workout(2 * MINUTE))
        .rest(30 * SECOND)
        .signal(&bell)
        .difficulty(Difficulty::Medium)
        .call();

    let boxing_6x3m_1m = Sequence::rounds()
        .name("6x3m")
        .rounds(6 * ROUNDS)
        .workout(item::BOXING_ROUND.workout(3 * MINUTE))
        .rest(MINUTE)
        .signal(&bell)
        .difficulty(Difficulty::Hard)
        .call();

    let stamina_jab_cross_hook = Sequence::repeat()
        .name("30s: 1 | 2 | 1-2 | 1-2-3")
        .exercises(Exercises::from(vec![
            "Jab (1)",
            "Cross (2)",
            "Jab | Cross (1-2)",
            "Jab | Cross | Hook (1-2-3)",
        ]))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Medium)
        .signal(&bell)
        .call();

    let stamina_jab_jab_cross_cross = Sequence::repeat()
        .name("30s: 1 | 1-1 | 1-1-2 | 1-1-2-2")
        .exercises(Exercises::from(vec![
            "Jab (1)",
            "Double Jab (1-1)",
            "Double Jab | Cross (1-1-2)",
            "Double Jab | Cross | Cross",
        ]))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Medium)
        .signal(&bell)
        .call();

    let stamina_jab_cross_hook_cross = Sequence::repeat()
        .name("30s: 1 | 1-2 | 1-2-3 | 1-2-3-2")
        .exercises(Exercises::from(vec![
            "Jab (1)",
            "Jab | Cross (1-2)",
            "Jab | Cross | Hook (1-2-3)",
            "Jab | Cross | Hook | Cross (1-2-3-2)",
        ]))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Medium)
        .signal(&bell)
        .call();

    let stamina_roll_left = Sequence::repeat()
        .name("30s: 1-2-3 | 1-2-3-ROLL | 1-2-3-ROLL-3 | 1-2-3-ROLL-3-2")
        .exercises(Exercises::from(vec![
            "Jab | Cross | Hook",
            "Jab | Cross | Hook | ROLL",
            "Jab | Cross | Hook | ROLL | Hook",
            "Jab | Cross | Hook | ROLL | Hook | Cross",
        ]))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Hard)
        .signal(&bell)
        .call();

    let stamina_roll_right = Sequence::repeat()
        .name("30s: 1-2-ROLL | 1-2-ROLL-2 | 1-2-ROLL-2-3 | 1-2-ROLL-2-3-2")
        .exercises(Exercises::from(vec![
            "Jab | Cross | ROLL",
            "Jab | Cross | ROLL | Cross",
            "Jab | Cross | ROLL | Cross | Hook",
            "Jab | Cross | ROLL | Cross | Hook | Cross",
        ]))
        .workout(30 * SECOND)
        .rounds(4 * ROUNDS)
        .rest(MINUTE)
        .difficulty(Difficulty::Hard)
        .signal(&bell)
        .call();

    let random_training = Sequence::random()
        .name("Random training (30s each) 🎲")
        .workouts(&[
            item::JUMPING_JACK.workout(30 * SECOND),
            item::PULL_UP.workout(30 * SECOND),
            item::PLANK.workout(30 * SECOND),
            item::SIDE_PLANK.workout(30 * SECOND),
            item::JUMP_SQUAT.workout(30 * SECOND),
            item::BURPEE.workout(30 * SECOND),
            item::PUSH_UP.workout(30 * SECOND),
            item::ALTERNATE_LUNGE.workout(30 * SECOND),
            item::CRUNCHES.workout(30 * SECOND),
        ])
        .rest(30 * SECOND)
        .signal(&beep)
        .call();

    let hiit_4x = Sequence::rounds()
        .name("HiiT 20s (4x)")
        .rounds(4 * ROUNDS)
        .workout(item::WORKOUT.workout(20 * SECOND))
        .difficulty(Difficulty::Medium)
        .rest(10 * SECOND)
        .signal(&beep)
        .call();

    let hiit_8x = Sequence::rounds()
        .name("HiiT 20s (8x)")
        .rounds(8 * ROUNDS)
        .workout(item::WORKOUT.workout(20 * SECOND))
        .difficulty(Difficulty::Hard)
        .rest(10 * SECOND)
        .signal(&beep)
        .call();

    let _5mn = Sequence::workout()
        .name("5m")
        .workout(item::WORKOUT.workout(5 * MINUTE))
        .signal(&bell)
        .call();

    let _10mn = Sequence::workout()
        .name("10m")
        .workout(item::WORKOUT.workout(10 * MINUTE))
        .signal(&bell)
        .call();

    let _15mn = Sequence::workout()
        .name("15m")
        .workout(item::WORKOUT.workout(15 * MINUTE))
        .signal(&bell)
        .call();

    let mut state_signal = use_signal(|| state.clone());

    let mut timer = use_signal(|| {
        let mut timer = timer::Timer::new(
            preparation,
            &[
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
                _5mn,
                _10mn,
                _15mn,
            ],
        );
        if !sequence.is_empty() {
            timer.set_sequence_by_slug(&sequence)
        }
        timer
    });

    let _tick = use_resource(move || async move {
        loop {
            gloo::timers::future::TimeoutFuture::new(timer::DEFAULT_INTERVAL).await;
            if timer.write().tick() {
                gloo::timers::future::TimeoutFuture::new(timer::DEFAULT_INTERVAL).await;
            }
        }
    });

    rsx! {
        link { rel: "stylesheet", href: "tailwind.css" }
        div { class: "flex flex-row space-x-1 m-1 ",
            div { id: "left_panel", class: "space-y-1.5",
                if let Some(sequence) = timer.read().sequences().get() {
                    div { id: "controls", class: "flex justify-evenly",
                        button {
                            class: "rounded-full text-3xl",
                            onclick: move |_| timer.with_mut(|t| t.toggle()),
                            {timer.read().next_status().to_string()}
                        }
                        button {
                            class: "rounded-full text-3xl",
                            onclick: move |_| timer.with_mut(|t| t.restart_sequence()),
                            "♻"
                        }
                        button {
                            class: "rounded-full text-3xl",
                            onclick: move |_| timer.with_mut(|t| t.manual_previous()),
                            "⏪"
                        }
                        button {
                            class: "rounded-full text-3xl",
                            onclick: move |_| timer.with_mut(|t| t.manual_next()),
                            "⏩"
                        }
                        if sequence.shufflable() {
                            button {
                                class: "rounded-full text-3xl",
                                onclick: move |_| timer.with_mut(|t| t.shuffle()),
                                "🎲"
                            }
                        }
                        if !sequence.signal().sound().is_silent() {
                            button {
                                id: "toggle_signal",
                                class: "text-3xl",
                                onclick: move |_| state_signal.with_mut(|s| s.borrow_mut().toggle()),
                                {state_signal.read().borrow().next().to_string()}
                            }
                            button {
                                id: "emit_signal",
                                class: "text-3xl",
                                onclick: move |_| { timer.with(|t| { t.always_ring() }) },
                                "🛎"
                            }
                        }
                    }
                }
                select {
                    id: "sequences",
                    name: "Sequence",
                    class: "select select-success",
                    oninput: move |ev| {
                        if let Ok(index) = ev.data.value().parse::<usize>() {
                            timer
                                .with_mut(|t| {
                                    if let Some(slug) = t.set_sequence(index) {
                                        let nav = navigator();
                                        nav.push(Route::BoxingTimer {
                                            muted,
                                            prepare,
                                            sequence: slug,
                                        });
                                    }
                                });
                        }
                    },
                    option { disabled: true, selected: true, "Select a workout" }
                    for (index , sequence) in timer.read().sequences().iter().enumerate() {
                        option {
                            value: index.to_string(),
                            selected: timer.read().sequences().get().map(|s| s.name() == sequence.name()),
                            {sequence.to_string()}
                        }
                    }
                }
                if let Some(sequence) = timer.read().sequences().get() {
                    if !sequence.is_empty() {
                        ul {
                            id: "sequence",
                            class: "info flex-none p-2 rounded-xl bg-sky-900",
                            for (index , item) in sequence.iter().enumerate() {
                                li {
                                    class: "text-nowrap",
                                    class: if sequence.index() == Some(index) { "text-red-600" } else { "" },
                                    span { class: "text-sm", "{item}" }
                                }
                            }
                        }
                    }
                }
                div {
                    id: "informations",
                    class: "rounded-xl bg-sky-900 text-2xl p-2",
                    div { id: "elapsed",
                        "Elapsed: "
                        {timer.read().elapsed().to_string()}
                    }
                    if let Some(sequence) = timer.read().sequences().get() {
                        div { id: "workout",
                            "Workout: "
                            {sequence.workout_total().to_string()}
                        }
                        if !sequence.rest_total().is_zero() {
                            div { id: "rest",
                                "Rest: "
                                {sequence.rest_total().to_string()}
                            }
                        }
                        div { id: "left",
                            "Left: "
                            {sequence.left_total().to_string()}
                        }
                        if !sequence.rest_total().is_zero() {
                            div { id: "total",
                                "Total: "
                                {sequence.total().to_string()}
                            }
                        }
                    }
                }
                div { id: "sounds",
                    audio {
                        id: bell.to_string(),
                        src: asset!("/assets/Bell.mp3"),
                        preload: "auto",
                        autoplay: false,
                    }
                    audio {
                        id: beep.to_string(),
                        src: asset!("/assets/Beep.mp3"),
                        preload: "auto",
                        autoplay: false,
                    }
                }
            }
            div {
                id: "timer",
                class: "bg-blue-600 flex w-full items-center justify-center h-screen rounded-xl",
                div { class: "flex flex-col items-center justify-center",
                    div { id: "item", class: "text-9xl", {timer.read().label()} }
                    div { id: "counter", class: "text-9xl", {timer.read().left().to_string()} }
                    button {
                        class: "text-3xl",
                        onclick: move |_| timer.with_mut(|t| t.restart_item()),
                        "♻"
                    }
                }
            }
        }
    }
}
