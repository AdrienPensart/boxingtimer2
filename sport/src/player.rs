use crate::errors::ErrorKind;
use crate::sound::Sound;

pub trait Player {
    fn play(&self, sound: &Sound) -> Result<(), ErrorKind>;
    fn muted(&self, sound: &Sound) -> Option<Result<bool, ErrorKind>>;
    fn unmute(&self, sound: &Sound) -> Result<(), ErrorKind>;
}

impl std::fmt::Debug for dyn Player {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "{}", "derp")
        Ok(())
    }
}
