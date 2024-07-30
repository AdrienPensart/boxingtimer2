use js_sys::Error;
use std::str::FromStr;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

pub fn get_param_or<T: FromStr>(param: &str, default: T) -> T {
    web_sys::window()
        .and_then(|window| window.location().href().ok())
        .and_then(|href| web_sys::Url::new(&href).ok())
        .and_then(|url| url.search_params().get(param))
        .and_then(|param| param.parse::<T>().ok())
        .unwrap_or(default)
}

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

pub struct BoxingBell;

impl BoxingBell {
    pub fn muted() -> bool {
        match muted() {
            Ok(muted) => muted,
            Err(e) => {
                log::error!("unable to bell muted state {}", e.message());
                false
            }
        }
    }
    pub fn toggle() {
        match muted() {
            Ok(true) => Self::unmute(),
            Ok(false) => Self::mute(),
            Err(e) => {
                log::error!("unable to mute or unmute bell {}", e.message());
            }
        };
    }

    pub fn mute() {
        if mute().is_err() {
            log::error!("Unable to mute bell");
        } else {
            log::info!("Bell muted");
        }
    }

    pub fn unmute() {
        if unmute().is_err() {
            log::error!("Unable to unmute bell");
        } else {
            log::info!("Bell unmuted");
        }
    }

    pub fn play() {
        if play().is_err() {
            log::error!("Unable to play bell");
        }
    }
}
