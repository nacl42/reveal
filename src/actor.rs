use crate::point::Point;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: Point,
}

impl Actor {
    pub fn new<P>(kind: ActorKind, pos: P) -> Self
    where P: Into<Point>
    {
        Self {
            kind,
            pos: pos.into()
        }
    }
}

#[derive(Debug, Clone)]
pub enum ActorKind {
    Player,
    Cat,
    Dog,
    Townsfolk,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ActorId(pub usize);

pub type ActorMap = HashMap<ActorId, Actor>;
    
