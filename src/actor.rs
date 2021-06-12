use crate::point::Point;
use crate::item::ItemId;
use crate::idmap::{Id, IdMap};

#[derive(Debug, Clone)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: Point,
    pub ai: Option<ActorAI>,
    pub inventory: Inventory
}

pub type ActorId = Id<Actor>;
pub type ActorMap = IdMap<Actor>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ActorKind {
    Player,
    Cat,
    Dog,
    Townsfolk,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ActorAI {
    DoNothing,
    WanderAround,
}

pub type Inventory = Vec<ItemId>;

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



pub fn actor_index(actor: &Actor) -> usize {
    match actor.kind {
        ActorKind::Player => 2,
        ActorKind::Townsfolk => 3,
        _ => 1
    }
}
