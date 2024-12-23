use crate::defaults;
use crate::duration::DurationExt;
use crate::routes::Route;
use crate::sequence::Sequence;
use crate::signal;
use crate::signal::{SoundSignal, TimerState};
use crate::sound::Sound;
use crate::timer::Timer;
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Clone)]
pub struct WebGlobal {
    pub timer: dioxus::signals::Signal<Timer>,
    pub state: dioxus::signals::Signal<Rc<RefCell<TimerState>>>,
    pub sequences: Vec<Sequence>,
    pub bell: SoundSignal,
    pub beep: SoundSignal,
}

impl WebGlobal {
    pub fn new(muted: bool, prepare: u64, sequence: Option<String>) -> Self {
        let state = if muted {
            TimerState::Disabled
        } else {
            TimerState::Enabled
        };
        let prepare = if prepare == 0 {
            defaults::PREPARE
        } else {
            prepare
        };
        let state = Rc::new(RefCell::new(state));

        let silent = SoundSignal::new(Sound::Silent, state.clone());
        let bell = SoundSignal::new(Sound::Bell, state.clone());
        let beep = SoundSignal::new(Sound::Beep, state.clone());
        let sequences = defaults::default_sequences(&bell, &beep, &silent);

        let mut timer = use_signal(|| {
            let mut timer = Timer::new(std::time::Duration::from_secs(prepare), &sequences);
            if let Some(sequence) = sequence {
                timer.set_sequence_by_slug(&sequence)
            }
            timer
        });

        let _tick = use_resource(move || async move {
            loop {
                gloo::timers::future::TimeoutFuture::new(defaults::DEFAULT_INTERVAL).await;
                if timer.write().tick() {
                    gloo::timers::future::TimeoutFuture::new(defaults::DEFAULT_INTERVAL).await;
                }
            }
        });

        Self {
            state: use_signal(|| state.clone()),
            sequences,
            bell,
            beep,
            timer,
        }
    }
}

#[component]
pub fn WebHome(muted: bool, prepare: u64, sequence: String) -> Element {
    let global = WebGlobal::new(muted, prepare, Some(sequence));
    let mut global = use_context_provider(|| global);
    rsx! {
        div { class: "flex flex-row space-x-1 m-1 ",
            div { id: "left_panel", class: "space-y-1.5",
                WebControls {}
                select {
                    id: "sequences",
                    name: "Sequence",
                    class: "select select-success",
                    oninput: move |ev| {
                        if let Ok(index) = ev.data.value().parse::<usize>() {
                            global
                                .timer
                                .with_mut(|t| {
                                    if let Some(slug) = t.set_sequence(index) {
                                        let nav = navigator();
                                        nav.push(Route::WebHome {
                                            muted,
                                            prepare,
                                            sequence: slug,
                                        });
                                    }
                                });
                        }
                    },
                    option { disabled: true, selected: true, "Select a workout" }
                    for (index , sequence) in global.timer.read().sequences().iter().enumerate() {
                        option {
                            id: format!("sequence_{}", sequence.slug()),
                            value: index.to_string(),
                            selected: global.timer.read().sequences().get().map(|s| s.name() == sequence.name()),
                            {sequence.to_string()}
                        }
                    }
                }
                if let Some(sequence) = global.timer.read().sequences().get() {
                    if !sequence.is_empty() {
                        ul {
                            id: "sequence",
                            class: "info flex-none p-2 rounded-xl bg-sky-900",
                            for (index , workout) in sequence.iter().enumerate() {
                                li {
                                    id: format!("workout_{}", workout.item().slug()),
                                    class: "text-nowrap",
                                    class: if sequence.index() == Some(index) { "text-red-600" } else { "" },
                                    span { class: "text-sm", "{workout}" }
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
                        {global.timer.read().elapsed().to_string()}
                    }
                    if let Some(sequence) = global.timer.read().sequences().get() {
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
                div { class: "flex justify-center text-2xl",
                    Link { id: "mobile_home", to: Route::MobileHome {}, "Mobile Home" }
                }
                signal::Sounds { bell: global.bell, beep: global.beep }
            }
            div {
                id: "timer",
                class: "bg-blue-600 flex w-full items-center justify-center h-screen rounded-xl",
                div { class: "flex flex-col items-center justify-center",
                    div { id: "workout", class: "text-9xl", {global.timer.read().label()} }
                    div { id: "counter", class: "text-9xl",
                        {global.timer.read().left().to_string()}
                    }
                    button {
                        id: "restart_workout",
                        class: "text-3xl",
                        onclick: move |_| global.timer.with_mut(|t| t.restart_workout()),
                        "♻"
                    }
                }
            }
        }
    }
}

#[component]
pub fn WebControls() -> Element {
    let mut global = use_context::<WebGlobal>();
    rsx! {
        if let Some(sequence) = global.timer.read().sequences().get() {
            div { id: "controls", class: "flex justify-evenly",
                button {
                    id: "toggle_timer",
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.toggle()),
                    {global.timer.read().next_status().to_string()}
                }
                button {
                    id: "restart_sequence",
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.restart_sequence()),
                    {defaults::RESTART_SEQUENCE}
                }
                button {
                    id: "previous_workout",
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.manual_previous()),
                    {defaults::PREVIOUS_ITEM}
                }
                button {
                    id: "next_workout",
                    class: "rounded-full text-3xl",
                    onclick: move |_| global.timer.with_mut(|t| t.manual_next()),
                    {defaults::NEXT_ITEM}
                }
                if sequence.shufflable() {
                    button {
                        id: "randomize",
                        class: "rounded-full text-3xl",
                        onclick: move |_| global.timer.with_mut(|t| t.shuffle()),
                        {defaults::RANDOMIZE}
                    }
                }
                if !sequence.signal().sound().is_silent() {
                    button {
                        id: "toggle_signal",
                        class: "text-3xl",
                        onclick: move |_| global.state.with_mut(|s| s.borrow_mut().toggle()),
                        {global.state.read().borrow().next().to_string()}
                    }
                    button {
                        id: "emit_signal",
                        class: "text-3xl",
                        onclick: move |_| { global.timer.with(|t| { t.always_ring() }) },
                        {defaults::SIGNAL}
                    }
                }
            }
        }
    }
}
