use crate::bell::BELL_ID;
use crate::errors::TimerErrorKind;
use gloo::console::log;
use wasm_bindgen::JsCast;

pub fn muted() -> Result<bool, TimerErrorKind> {
    Ok(audio()?.muted())
}

pub fn mute() -> Result<(), TimerErrorKind> {
    audio()?.set_muted(true);
    Ok(())
}

pub fn unmute() -> Result<(), TimerErrorKind> {
    audio()?.set_muted(false);
    Ok(())
}

fn audio() -> Result<web_sys::HtmlAudioElement, TimerErrorKind> {
    let window = web_sys::window().ok_or_else(|| TimerErrorKind::WindowError)?;
    let document = window
        .document()
        .ok_or_else(|| TimerErrorKind::DocumentError)?;
    let bell = document
        .get_element_by_id(BELL_ID)
        .ok_or_else(|| TimerErrorKind::BellError)?;
    bell.dyn_into::<web_sys::HtmlAudioElement>()
        .map_err(TimerErrorKind::AudioError)
}

pub fn play(always: bool) -> Result<(), TimerErrorKind> {
    let was_muted = muted()?;
    if always {
        unmute()?;
    }
    let promise = audio()?.play()?;
    wasm_bindgen_futures::spawn_local(async move {
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        let result = future.await;
        match result {
            Ok(_) => {
                if always && was_muted {
                    log!("re-mute");
                    if let Err(error) = mute() {
                        log!("failed to re-mute", error.to_string())
                    }
                }
            }
            Err(error) => {
                log!("failed to await future", error)
            }
        }
    });
    Ok(())
}
