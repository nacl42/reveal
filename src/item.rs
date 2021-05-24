use crate::point::Point;
use std::collections::HashMap;

pub type ItemMap = HashMap<ItemId, Item>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ItemId(pub u64);

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub pos: Option<Point>
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Money(u32),
    Wand,
}
