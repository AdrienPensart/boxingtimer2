use derive_more::Display;

#[derive(Display, Debug, PartialEq, Eq, Clone)]
pub enum Status {
    #[display("Pause")]
    Paused,
    #[display("Start")]
    Running,
}

impl Default for Status {
    fn default() -> Self {
        Self::Paused
    }
}

impl Status {
    pub fn toggle(&mut self) -> &mut Self {
        *self = self.next().clone();
        self
    }
    pub fn next(&self) -> &Self {
        match self {
            Self::Paused => &Self::Running,
            Self::Running => &Self::Paused,
        }
    }
    pub fn paused(&self) -> bool {
        matches!(self, Self::Paused)
    }
    pub fn running(&self) -> bool {
        matches!(self, Self::Running)
    }
}
