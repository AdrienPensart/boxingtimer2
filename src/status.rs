use derive_more::Display;

#[derive(Display, Debug, PartialEq, Eq, Clone)]
pub enum Status {
    #[display("🛑")]
    Paused,
    #[display("🟢")]
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
    #[must_use]
    pub fn next(&self) -> &Self {
        match self {
            Self::Paused => &Self::Running,
            Self::Running => &Self::Paused,
        }
    }
    #[must_use]
    pub fn paused(&self) -> bool {
        matches!(self, Self::Paused)
    }
    #[must_use]
    pub fn running(&self) -> bool {
        matches!(self, Self::Running)
    }
}
