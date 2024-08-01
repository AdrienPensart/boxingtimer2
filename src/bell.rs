use crate::helpers;

#[derive(Default)]
pub struct Bell {}

impl Bell {
    pub fn new() -> Self {
        Self {}
    }
    pub fn muted() -> bool {
        match helpers::muted() {
            Ok(muted) => muted,
            Err(e) => {
                log::error!("unable to get bell state {}", e.message());
                false
            }
        }
    }
    pub fn toggle(&self) {
        if Self::muted() {
            Self::unmute()
        } else {
            Self::mute()
        }
    }
    pub fn mute() {
        if helpers::mute().is_err() {
            log::error!("unable to mute bell");
        } else {
            log::info!("bell muted");
        }
    }
    pub fn unmute() {
        if helpers::unmute().is_err() {
            log::error!("unable to unmute bell");
        } else {
            log::info!("bell unmuted");
        }
    }
    pub fn ring() {
        if helpers::play().is_ok() {
            log::error!("unable to play bell");
        }
    }
    pub fn next_label(&self) -> String {
        if Self::muted() {
            "Unmute".to_string()
        } else {
            "Mute".to_string()
        }
    }
}
