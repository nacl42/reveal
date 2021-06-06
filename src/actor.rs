use crate::point::Point;
use crate::item::ItemId;
use crate::idmap::IdMap;

use crate::id::Id;

#[derive(Debug, Clone)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: Point,
    pub ai: Option<ActorAI>,
    pub inventory: Vec<ItemId>
}

impl Actor {
    pub fn new<P>(kind: ActorKind, pos: P) -> Self
    where P: Into<Point>
    {
        Self {
            kind,
            pos: pos.into(),
            ai: None,
            inventory: Vec::new()
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
}


#[derive(Debug, Clone)]
pub enum ActorAI {
    DoNothing,
    WanderAround,
}

pub type ActorId = Id<Actor>;
pub type ActorMap = IdMap<Actor>;
