use crate::errors::TimerErrorKind;
use derive_more::Display;
use dioxus_logger::tracing::info;
use web_sys::wasm_bindgen::JsCast;

#[derive(Debug, Display, Default, Clone, Eq, PartialEq)]
pub enum Sound {
    Silent,
    #[default]
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
    pub fn asset(&self) -> String {
        format!("{self}.mp3")
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
            .ok_or_else(|| TimerErrorKind::SoundError)?;
        sound
            .dyn_into::<web_sys::HtmlAudioElement>()
            .map_err(TimerErrorKind::AudioError)
    }

    pub fn play(&self) -> Result<(), TimerErrorKind> {
        let promise = self.audio()?.play()?;
        wasm_bindgen_futures::spawn_local(async move {
            let future = wasm_bindgen_futures::JsFuture::from(promise);
            if let Err(err) = future.await {
                info!(
                    "failed to await future: {}",
                    err.as_string().unwrap_or("unknown error".to_string())
                )
            }
        });
        Ok(())
    }
}
