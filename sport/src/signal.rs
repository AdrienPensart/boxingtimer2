use crate::player::Player;
use crate::sound::Sound;
use derive_more::Display;
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
    }
    #[must_use]
    pub fn next(&self) -> Self {
        match self {
            Self::Disabled => Self::Enabled,
            Self::Enabled => Self::Disabled,
        }
    }
    #[must_use]
    pub fn next_title(&self) -> String {
        match self.next() {
            Self::Disabled => "Disable sound".to_string(),
            Self::Enabled => "Enable sound".to_string(),
        }
    }
    #[must_use]
    pub fn enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }
    #[must_use]
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
    #[must_use]
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
    pub fn ring(&self, sound: &Sound, player: &dyn Player) {
        if self.enabled() {
            if let Err(_error) = player.play(sound) {}
        }
    }
    pub fn always_ring(&self, sound: &Sound, player: &dyn Player) {
        if let Err(_error) = player.play(sound) {}
    }
    #[must_use]
    pub fn state(&self) -> SoundState {
        self.state.borrow().clone()
    }
    #[must_use]
    pub fn next(&self) -> SoundState {
        self.state.borrow().next()
    }
    #[must_use]
    pub fn enabled(&self) -> bool {
        self.state.borrow().enabled()
    }
    #[must_use]
    pub fn disabled(&self) -> bool {
        !self.enabled()
    }
}
