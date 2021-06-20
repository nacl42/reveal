//! Experimental implementation of a simple Skill system.
//!
//! We wouldn't need this if we had an ECS.
//!

#[derive(Debug, Clone)]
pub struct Skill {
    pub kind: SkillKind,
    pub duration: SkillDuration
}

impl Skill {
    pub fn new(kind: SkillKind) -> Self {
        Self {
            kind,
            duration: SkillDuration::Permanent
        }
    }

    pub fn new_temporary<D>(kind: SkillKind, duration: D) -> Self
    where D: Into<SkillDuration>
    {
        Self {
            kind,
            duration: duration.into()
        }
    }

    pub fn description(&self) -> String {
        let kind = match self.kind {
            SkillKind::Swim => "swimming",
            SkillKind::Climb => "climbing"
        };
        format!("{}", kind)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SkillKind {
    Swim,
    Climb,
}

pub type GameTime = usize;

#[derive(Debug, Clone)]
pub enum SkillDuration {
    Permanent,
    Temporary { duration: GameTime },
}

impl From<SkillKind> for Skill {
    fn from (kind: SkillKind) -> Self {
        Self {
            kind,
            duration: SkillDuration::Permanent
        }
    }
}

impl From<GameTime> for SkillDuration {
    fn from(time: GameTime) -> Self {
        SkillDuration::Temporary { duration: time }
    }
}
