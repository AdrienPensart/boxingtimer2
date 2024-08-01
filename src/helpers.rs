use js_sys::Error;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

pub fn muted() -> Result<bool, Error> {
    Ok(audio()?.muted())
}

pub fn mute() -> Result<(), Error> {
    audio()?.set_muted(true);
    Ok(())
}

pub fn unmute() -> Result<(), Error> {
    audio()?.set_muted(false);
    Ok(())
}

fn audio() -> Result<web_sys::HtmlAudioElement, Error> {
    let window = web_sys::window().ok_or_else(|| Error::new("cannot get window"))?;
    let document = window
        .document()
        .ok_or_else(|| Error::new("cannot get document"))?;
    let bell = document
        .get_element_by_id("bell")
        .ok_or_else(|| Error::new("cannot get bell element"))?;
    bell.dyn_into::<web_sys::HtmlAudioElement>()
        .map_err(|_| Error::new("cannot convert to audio element"))
}

pub fn play() -> Result<(), Error> {
    if !muted()? {
        let promise = audio()?.play()?;
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        spawn_local(async move {
            let _ = future.await.map_err(|e| {
                if !e.as_string().unwrap().contains("NotAllowedError") {
                    log::info!("failed to await future {e:?}")
                }
            });
        });
    }
    Ok(())
}
