

#[derive(Debug, Clone)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: (usize, usize),
}

#[derive(Debug, Clone)]
pub enum ActorKind {
    Player,
    Cat,
    Dog,
    Townsfolk,
}

