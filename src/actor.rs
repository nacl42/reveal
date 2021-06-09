use crate::point::Point;
use crate::item::ItemId;
use crate::idmap::{Id, IdMap};
use crate::game::{ActorKind, ActorAI};

#[derive(Debug, Clone)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: Point,
    pub ai: Option<ActorAI>,
    pub inventory: Inventory
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


pub type ActorId = Id<Actor>;
pub type ActorMap = IdMap<Actor>;


pub type Inventory = Vec<ItemId>;
