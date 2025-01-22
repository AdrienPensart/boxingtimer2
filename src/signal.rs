use crate::audio;
use derive_more::Display;
use dioxus::logger::tracing::info;
use sport::sound::Sound;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Display, Default, Clone, Eq, PartialEq)]
pub enum SoundState {
    #[default]
    #[display("🔊")]
    Enabled,
    #[display("🔇")]
    Disabled,
}

impl SoundState {
    pub fn toggle(&mut self) {
        *self = self.next();
        info!("signal {self}");
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

pub type SharedSoundState = Rc<RefCell<SoundState>>;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SoundSignal {
    state: SharedSoundState,
}

impl SoundSignal {
    pub fn from_muted(muted: bool) -> Self {
        let state = if muted {
            SoundState::Disabled
        } else {
            SoundState::Enabled
        };
        Self::new(Rc::new(RefCell::new(state)))
    }
    pub fn new(state: SharedSoundState) -> Self {
        Self { state }
    }
    pub fn toggle(&mut self) {
        self.state.borrow_mut().toggle();
    }
    pub fn ring(&self, sound: &Sound) {
        if self.enabled() {
            info!("signal {sound} ring");
            if let Err(error) = audio::play(sound) {
                info!("unable to play {sound} : {error}");
            }
        }
    }
    pub fn always_ring(&self, sound: &Sound) {
        info!("signal {sound} ring (always)");
        if let Err(error) = audio::play(sound) {
            info!("unable to play {sound} (always): {error}");
        }
    }
    pub fn state(&self) -> SoundState {
        self.state.borrow().clone()
    }
    pub fn next(&self) -> SoundState {
        self.state.borrow().next()
    }
    pub fn enabled(&self) -> bool {
        self.state.borrow().enabled()
    }
    pub fn disabled(&self) -> bool {
        !self.enabled()
    }
}
