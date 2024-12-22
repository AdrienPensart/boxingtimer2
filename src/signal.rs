use crate::sound::Sound;
use derive_more::Display;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Display, Default, Clone, Eq, PartialEq)]
pub enum TimerState {
    #[default]
    #[display("🔊")]
    Enabled,
    #[display("🔇")]
    Disabled,
}

impl TimerState {
    pub fn toggle(&mut self) {
        *self = self.next()
    }
    pub fn next(&self) -> Self {
        match self {
            Self::Disabled => Self::Enabled,
            Self::Enabled => Self::Disabled,
        }
    }
    pub fn enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }
    pub fn disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }
}

#[derive(Debug, Display, Default, Clone, Eq, PartialEq)]
#[display("{sound}")]
pub struct SoundSignal {
    sound: Sound,
    state: Rc<RefCell<TimerState>>,
}

impl SoundSignal {
    pub fn none() -> Self {
        Self::new(Sound::Silent, Rc::new(RefCell::new(TimerState::Disabled)))
    }
    pub fn new(sound: Sound, state: Rc<RefCell<TimerState>>) -> Self {
        Self { sound, state }
    }
    pub fn toggle(&mut self) {
        self.state.borrow_mut().toggle();
    }
    pub fn ring(&self) {
        if self.enabled() {
            info!("signal {} ring", self.sound);
            if let Err(error) = self.sound.play() {
                info!("unable to play {} : {error}", self.sound);
            }
        }
    }
    pub fn always_ring(&self) {
        info!("signal {} ring (always)", self.sound);
        if let Err(error) = self.sound.play() {
            info!("unable to play {} (always): {error}", self.sound);
        }
    }
    pub fn next(&self) -> TimerState {
        self.state.borrow().next()
    }
    pub fn enabled(&self) -> bool {
        self.state.borrow().enabled()
    }
    pub fn disabled(&self) -> bool {
        !self.enabled()
    }
    pub fn sound(&self) -> &Sound {
        &self.sound
    }
}

#[component]
pub fn Sounds(bell: SoundSignal, beep: SoundSignal) -> Element {
    rsx! {
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
}
