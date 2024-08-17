use crate::item::ItemBuilderError;

#[derive(thiserror::Error, Debug)]
pub enum TimerErrorKind {
    #[error("Item error: {0}")]
    ItemBuilderError(#[from] ItemBuilderError),
    #[error("JS error: {0:?}")]
    JsError(js_sys::Error),
    #[error("Cannot get audio element: {0:?}")]
    AudioError(web_sys::Element),
    #[error("Cannot ring bell: {0:?}")]
    PlayError(web_sys::wasm_bindgen::JsValue),
    #[error("Cannot get window")]
    WindowError,
    #[error("Cannot get document")]
    DocumentError,
    #[error("Cannot get bell")]
    BellError,
}

impl From<js_sys::Error> for TimerErrorKind {
    fn from(err: js_sys::Error) -> Self {
        TimerErrorKind::JsError(err)
    }
}

impl From<web_sys::wasm_bindgen::JsValue> for TimerErrorKind {
    fn from(value: web_sys::wasm_bindgen::JsValue) -> Self {
        TimerErrorKind::PlayError(value)
    }
}
