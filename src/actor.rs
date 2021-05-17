

#[derive(Debug, Clone)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: (i16, i16),
}

#[derive(Debug, Clone)]
pub enum ActorKind {
    Player,
    Cat,
    Dog,
    Townsfolk,
}

