use std::{cell::RefCell, rc::Rc};

use crate::sound::Sound;
use derive_more::Display;
use dioxus_logger::tracing::info;

#[derive(Debug, Display, Default, Clone, Eq, PartialEq)]
pub enum State {
    #[default]
    Enabled,
    Disabled,
}

impl State {
    pub fn toggle(&mut self) {
        *self = self.next()
    }
    pub fn next(&self) -> Self {
        match self {
            Self::Disabled => Self::Enabled,
            Self::Enabled => Self::Disabled,
        }
    }
    pub fn next_label(&self) -> String {
        match self {
            State::Disabled => "Unmute".to_string(),
            State::Enabled => "Mute".to_string(),
        }
    }
    pub fn enabled(&self) -> bool {
        match self {
            State::Disabled => false,
            State::Enabled => true,
        }
    }
    pub fn disabled(&self) -> bool {
        !self.enabled()
    }
}

#[derive(Debug, Display, Default, Clone, Eq, PartialEq)]
#[display("{sound}")]
pub struct Signal {
    sound: Sound,
    state: Rc<RefCell<State>>,
}

impl Signal {
    pub fn new(sound: Sound, state: Rc<RefCell<State>>) -> Self {
        // let next_state = match *state.borrow() {
        // if *state.borrow() == State::Enabled {
        //     Self::unmute(&sound);
        // };
        // *state.borrow_mut() = next_state;
        Self { sound, state }
    }
    pub fn asset(&self) -> String {
        self.sound.asset()
    }
    pub fn toggle(&mut self) {
        self.state.borrow_mut().toggle();
    }
    // fn disable() {
    //     match sound.mute() {
    //         Ok(()) => {
    //             info!("signal {sound} muted");
    //             State::Disabled
    //         }
    //         Err(error) => {
    //             info!("unable to mute signal {sound} : {error}");
    //             State::Enabled
    //         }
    //     }
    // }
    // fn enable(sound: &Sound) -> State {
    //     match sound.unmute() {
    //         Ok(()) => {
    //             info!("signal {sound} unmuted");
    //             State::Enabled
    //         }
    //         Err(error) => {
    //             info!("unable to unmute signal {sound} : {error}");
    //             State::Disabled
    //         }
    //     }
    // }
    // fn enable(sound: &Sound) -> State {
    //     match sound.unmute() {
    //         Ok(()) => {
    //             info!("signal {sound} unmuted");
    //             State::Enabled
    //         }
    //         Err(error) => {
    //             info!("unable to unmute signal {sound} : {error}");
    //             State::Disabled
    //         }
    //     }
    // }
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
    pub fn next_label(&self) -> String {
        self.state.borrow().next_label()
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
