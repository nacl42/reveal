use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: Point,
}

#[derive(Debug, Clone)]
pub enum ActorKind {
    Player,
    Cat,
    Dog,
    Townsfolk,
}

