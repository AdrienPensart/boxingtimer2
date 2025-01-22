use crate::errors::TimerErrorKind;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use sport::sound::Sound;
use web_sys::wasm_bindgen::JsCast;

pub fn muted(sound: &Sound) -> Option<Result<bool, TimerErrorKind>> {
    if !sound.is_silent() {
        match audio(sound) {
            Ok(audio) => Some(Ok(audio.muted())),
            Err(err) => Some(Err(err)),
        }
    } else {
        None
    }
}

pub fn unmute(sound: &Sound) -> Result<(), TimerErrorKind> {
    audio(sound)?.set_muted(false);
    Ok(())
}

fn audio(sound: &Sound) -> Result<web_sys::HtmlAudioElement, TimerErrorKind> {
    let window = web_sys::window().ok_or_else(|| TimerErrorKind::WindowError)?;
    let document = window
        .document()
        .ok_or_else(|| TimerErrorKind::DocumentError)?;
    let sound = document
        .get_element_by_id(sound.to_string().as_str())
        .ok_or_else(|| TimerErrorKind::SoundError(sound.to_string()))?;
    sound
        .dyn_into::<web_sys::HtmlAudioElement>()
        .map_err(TimerErrorKind::AudioError)
}

pub fn play(sound: &Sound) -> Result<(), TimerErrorKind> {
    if sound.is_silent() {
        gloo::dialogs::alert("sequence is silent");
        return Ok(());
    }
    let promise = audio(sound)?.play()?;
    wasm_bindgen_futures::spawn_local(async move {
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        if let Err(err) = future.await {
            info!("failed to await future: {:?}", err)
        }
    });
    Ok(())
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
