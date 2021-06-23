use crate::{
    point::{Point, PointSet},
    item::ItemId,
    idmap::{Id, IdMap},
    skill::{Skill, SkillKind, SkillDuration}
};

#[derive(Debug, Clone)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: Point,
    pub health: Attribute,
    pub coins: u16,
    pub ai: ActorAI,
    pub inventory: Inventory,
    pub visited: PointSet,
    pub skills: Vec<Skill>
}

pub type ActorId = Id<Actor>;
pub type ActorMap = IdMap<Actor>;

pub type StatusValue = u16;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Attribute {
    pub value: u16,
    pub max: u16
}

impl From<(u16, u16)> for Attribute {
    fn from((value, max): (u16, u16)) -> Self {
        Self {
            value,
            max
        }
    }
}

impl From<u16> for Attribute {
    fn from(value: u16) -> Self {
        Self {
            value,
            max: value
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ActorKind {
    Player,
    Cat,
    Dog,
    Townsfolk,
    Shopkeeper
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ActorAI {
    DoNothing,
    WanderAround,
}

pub type Inventory = Vec<ItemId>;

impl Actor {
    pub fn new<P, A>(kind: ActorKind, pos: P, health: A) -> Self
    where P: Into<Point>, A: Into<Attribute>
    {
        Self {
            kind,
            pos: pos.into(),
            ai: ActorAI::WanderAround,
            health: health.into(),
            coins: 0,
            inventory: Vec::new(),
            visited: PointSet::new(),
            skills: Vec::new()
        }
    }

    pub fn with_ai(mut self, ai: ActorAI) -> Self {
        self.ai = ai;
        self
    }
    
    pub fn is_npc(&self) -> bool {
        match self.kind {
            ActorKind::Player => false,
            _ => true
        }
    }

    pub fn has_skill(&self, kind: &SkillKind) -> bool {
        self.skills.iter().
            any(|skill| skill.kind == *kind)
    }

    pub fn description(&self) -> String {
        match self.kind {
            ActorKind::Player => format!("player"),
            ActorKind::Cat => format!("a cat"),
            ActorKind::Dog => format!("a dog"),
            ActorKind::Townsfolk => format!("a villager"),
            ActorKind::Shopkeeper => format!("a shopkeeper"),
        }
    }
}



pub fn actor_index(actor: &Actor) -> usize {
    match actor.kind {
        ActorKind::Player => 2,
        ActorKind::Townsfolk => 3,
        ActorKind::Shopkeeper => 0,
        _ => 1
    }
}
