use crate::point::Point;
use crate::actor::{ActorId};
use crate::idmap::{Id, IdMap};

pub type ItemId = Id<Item>;
pub type ItemMap = IdMap<Item>;


#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub pos: Option<Point>,
    pub owner: Option<ActorId>
}

impl Item {
    pub fn new(kind: ItemKind) -> Self {
        Self {
            kind,
            pos: None,
            owner: None
        }
    }

    pub fn with_pos<P>(mut self, pos: P) -> Self
    where P: Into<Point>
    {
        self.pos = Some(pos.into());
        self
    }

    pub fn with_owner(mut self, owner: ActorId) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn description(&self) -> String {
        match self.kind {
            ItemKind::Wand => String::from("a magical wand"),
            ItemKind::Money(x) => format!("{} coins of gold", x)
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Money(u32),
    Wand,
}
