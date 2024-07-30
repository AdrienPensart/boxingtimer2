use crate::difficulty::Difficulty;
use crate::duration::Duration;
use crate::errors::BoxingTimerErrorKind;
use crate::tag::Tag;
use derive_builder::Builder;
use derive_new::new;

#[derive(new, Default, Builder, Clone, Debug, Eq, PartialEq)]
#[builder(setter(into))]
pub struct Item {
    #[new(value = "None")]
    #[builder(setter(strip_option), default)]
    name: Option<String>,
    #[new(value = "Difficulty::Unknown")]
    #[builder(default)]
    difficulty: Difficulty,
    duration: Duration,
    #[new(value = "vec![]")]
    #[builder(default)]
    tags: Vec<Tag>,
    #[new(value = "false")]
    #[builder(default)]
    waiting: bool,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if let Some(name) = &self.name {
            write!(f, "{name}")?;
            name.clone()
        } else if self.tags.len() == 1 {
            write!(f, "{}", self.tags[0])?;
            self.tags[0].to_string()
        } else {
            "".to_string()
        };
        if self.difficulty == Difficulty::Unknown {
            write!(f, " : {}", self.duration)?;
        } else {
            write!(f, " - {} : {}", self.difficulty, self.duration)?;
        }

        if !self.tags.is_empty() {
            let tags = self
                .tags
                .iter()
                .map(|&t| t.to_string())
                .collect::<Vec<_>>()
                .join(" / ");
            if name != tags {
                write!(f, " ({tags})")?;
            }
        }
        Ok(())
    }
}

impl Item {
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
    pub fn duration(&self) -> &Duration {
        &self.duration
    }
    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }
    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }
    pub fn waiting(&self) -> bool {
        self.waiting
    }
    pub fn prepare(&self) -> bool {
        self.tags.contains(&Tag::Prepare)
    }
}

pub fn Boxing(duration: Duration) -> Result<Item, BoxingTimerErrorKind> {
    Ok(ItemBuilder::default()
        .difficulty(Difficulty::Medium)
        .duration(duration)
        .tags([Tag::Boxing])
        .build()?)
}

pub fn Prepare(duration: Duration) -> Result<Item, BoxingTimerErrorKind> {
    Ok(ItemBuilder::default()
        .waiting(true)
        .tags([Tag::Prepare])
        .duration(duration)
        .build()?)
}

pub fn Maintain(duration: Duration) -> Result<Item, BoxingTimerErrorKind> {
    Ok(ItemBuilder::default()
        .name("Maintain")
        .difficulty(Difficulty::Hard)
        .waiting(true)
        .duration(duration)
        .build()?)
}

pub fn Contract(duration: Duration) -> Result<Item, BoxingTimerErrorKind> {
    Ok(ItemBuilder::default()
        .name("Contract")
        .difficulty(Difficulty::Hard)
        .waiting(true)
        .duration(duration)
        .build()?)
}

pub fn Rest(duration: Duration) -> Result<Item, BoxingTimerErrorKind> {
    Ok(ItemBuilder::default()
        .name("Rest")
        .waiting(true)
        .duration(duration)
        .build()?)
}

pub fn TheEnd(duration: Duration) -> Result<Item, BoxingTimerErrorKind> {
    Ok(ItemBuilder::default()
        .name("The End")
        .waiting(true)
        .duration(duration)
        .build()?)
}
