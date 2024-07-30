use derive_more::Display;

#[derive(Display, Debug, Default, PartialEq, Eq)]
pub enum Status {
    #[default]
    #[display(fmt = "Pause")]
    Paused,
    #[display(fmt = "Start")]
    Running,
}

impl Status {
    pub fn toggle(&mut self) {
        *self = self.next()
    }
    pub fn next(&self) -> Self {
        match self {
            Self::Paused => Self::Running,
            Self::Running => Self::Paused,
        }
    }
}
