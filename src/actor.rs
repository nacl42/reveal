use crate::point::Point;
use crate::item::ItemId;

use std::collections::HashMap;
use crate::id::Id;

#[derive(Debug, Clone)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: Point,
    pub inventory: Vec<ItemId>
}

impl Actor {
    pub fn new<P>(kind: ActorKind, pos: P) -> Self
    where P: Into<Point>
    {
        Self {
            kind,
            pos: pos.into(),
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

//#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
//pub struct ActorId(pub u64);

pub type ActorId = Id<Actor>;
pub type ActorMap = HashMap<ActorId, Actor>;
    
