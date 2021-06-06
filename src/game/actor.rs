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
