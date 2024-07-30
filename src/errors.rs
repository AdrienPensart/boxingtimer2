use crate::item::ItemBuilderError;

#[derive(thiserror::Error, Debug)]
pub enum BoxingTimerErrorKind {
    #[error("Item error: {0}")]
    ItemBuilderError(#[from] ItemBuilderError),
}
