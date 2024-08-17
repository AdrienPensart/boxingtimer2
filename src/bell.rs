use crate::helpers;
use gloo::console::log;

pub static BELL_ID: &str = "bell";

#[derive(Clone, Default, Eq, PartialEq)]
pub enum Bell {
    #[default]
    Enabled,
    Disabled,
}

impl Bell {
    pub fn new(muted: bool) -> Self {
        if muted {
            Self::mute()
        } else {
            Self::unmute()
        }
    }
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Disabled => Self::unmute(),
            Self::Enabled => Self::mute(),
        };
    }
    fn mute() -> Self {
        match helpers::mute() {
            Ok(()) => {
                log!("bell muted");
                Self::Disabled
            }
            Err(error) => {
                log!("unable to mute bell", error.to_string());
                Self::Enabled
            }
        }
    }
    fn unmute() -> Self {
        match helpers::unmute() {
            Ok(()) => {
                log!("bell unmuted");
                Self::Enabled
            }
            Err(error) => {
                log!("unable to unmute bell", error.to_string());
                Self::Disabled
            }
        }
    }
    pub fn ring(&self) {
        log!("ring");
        if self == &Self::Enabled {
            if let Err(error) = helpers::play(false) {
                log!("unable to play bell", error.to_string());
            }
        }
    }
    pub fn always_ring(&self) {
        log!("always ring");
        if self == &Self::Disabled {
            if let Err(error) = helpers::play(true) {
                log!("unable to play bell (always)", error.to_string());
            }
        } else {
            self.ring()
        }
    }
    pub fn next_label(&self) -> String {
        match self {
            Self::Disabled => "Unmute".to_string(),
            Self::Enabled => "Mute".to_string(),
        }
    }
}
