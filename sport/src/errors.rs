#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    #[error("Player error: {0}")]
    PlayerError(String),
    // #[error("Cannot get window")]
    // WindowError,
    // #[error("Cannot get document")]
    // DocumentError,
    // #[error("Cannot get sound of id: {0}")]
    // SoundError(String),
    #[error("Invalid sequence")]
    InvalidSequence,
}

// impl From<js_sys::Error> for ErrorKind {
//     fn from(err: js_sys::Error) -> Self {
//         ErrorKind::RuntimeError(err)
//     }
// }

// impl From<web_sys::wasm_bindgen::JsValue> for ErrorKind {
//     fn from(value: web_sys::wasm_bindgen::JsValue) -> Self {
//         ErrorKind::PlayError(value)
//     }
// }
