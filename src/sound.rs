use crate::errors::TimerErrorKind;
use derive_more::Display;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::JsCast;

#[derive(Debug, Display, Default, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Sound {
    #[default]
    Silent,
    Bell,
    Beep,
}

impl Sound {
    pub fn is_silent(&self) -> bool {
        matches!(self, Self::Silent)
    }
    pub fn is_bell(&self) -> bool {
        matches!(self, Self::Bell)
    }
    pub fn is_beep(&self) -> bool {
        matches!(self, Self::Beep)
    }
    pub fn muted(&self) -> Option<Result<bool, TimerErrorKind>> {
        if *self != Self::Silent {
            match self.audio() {
                Ok(audio) => Some(Ok(audio.muted())),
                Err(err) => Some(Err(err)),
            }
        } else {
            None
        }
    }
    pub fn unmute(&self) -> Result<(), TimerErrorKind> {
        self.audio()?.set_muted(false);
        Ok(())
    }
    fn audio(&self) -> Result<web_sys::HtmlAudioElement, TimerErrorKind> {
        let window = web_sys::window().ok_or_else(|| TimerErrorKind::WindowError)?;
        let document = window
            .document()
            .ok_or_else(|| TimerErrorKind::DocumentError)?;
        let sound = document
            .get_element_by_id(self.to_string().as_str())
            .ok_or_else(|| TimerErrorKind::SoundError(self.to_string()))?;
        sound
            .dyn_into::<web_sys::HtmlAudioElement>()
            .map_err(TimerErrorKind::AudioError)
    }
    pub fn play(&self) -> Result<(), TimerErrorKind> {
        if self.is_silent() {
            gloo::dialogs::alert("sequence is silent");
            return Ok(());
        }
        let promise = self.audio()?.play()?;
        wasm_bindgen_futures::spawn_local(async move {
            let future = wasm_bindgen_futures::JsFuture::from(promise);
            if let Err(err) = future.await {
                info!("failed to await future: {:?}", err)
            }
        });
        Ok(())
    }
}

#[component]
pub fn Sounds() -> Element {
    rsx! {
        div { id: "sounds",
            audio {
                id: Sound::Bell.to_string(),
                src: asset!("/assets/Bell.mp3"),
                preload: "auto",
                autoplay: false,
            }
            audio {
                id: Sound::Beep.to_string(),
                src: asset!("/assets/Beep.mp3"),
                preload: "auto",
                autoplay: false,
            }
        }
    }
}
