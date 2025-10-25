use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use sport::errors::ErrorKind;
use sport::player::Player;
use sport::sound::Sound;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct AudioPlayer;

impl Player for AudioPlayer {
    fn play(&self, sound: &Sound) -> Result<(), ErrorKind> {
        play(sound)
    }
    fn muted(&self, sound: &Sound) -> Option<Result<bool, ErrorKind>> {
        muted(sound)
    }
    fn unmute(&self, sound: &Sound) -> Result<(), ErrorKind> {
        unmute(sound)
    }
}

#[must_use]
pub fn muted(sound: &Sound) -> Option<Result<bool, ErrorKind>> {
    if sound.is_silent() {
        None
    } else {
        match audio(sound) {
            Ok(audio) => Some(Ok(audio.muted())),
            Err(err) => Some(Err(err)),
        }
    }
}

pub fn unmute(sound: &Sound) -> Result<(), ErrorKind> {
    audio(sound)?.set_muted(false);
    Ok(())
}

fn audio(sound: &Sound) -> Result<web_sys::HtmlAudioElement, ErrorKind> {
    let window = web_sys::window()
        .ok_or_else(|| ErrorKind::RuntimeError("cannot get window".to_string()))?;
    let document = window
        .document()
        .ok_or_else(|| ErrorKind::RuntimeError("cannot get document".to_string()))?;
    let sound = document
        .get_element_by_id(sound.to_string().as_str())
        .ok_or_else(|| {
            ErrorKind::RuntimeError(format!("cannot get sound element {}", sound.to_string()))
        })?;
    sound
        .dyn_into::<web_sys::HtmlAudioElement>()
        .map_err(|_| ErrorKind::PlayerError("cannot cast to HtmlAudioElement".to_string()))
}

pub fn play(sound: &Sound) -> Result<(), ErrorKind> {
    if sound.is_silent() {
        gloo::dialogs::alert("sequence is silent");
        return Ok(());
    }
    let promise = audio(sound)?.play().map_err(|err| {
        ErrorKind::PlayerError(format!(
            "cannot play sound {}: {:?}",
            sound.to_string(),
            err
        ))
    })?;
    wasm_bindgen_futures::spawn_local(async move {
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        if let Err(err) = future.await {
            info!("failed to await future: {:?}", err);
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
