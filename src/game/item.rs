//! Item customization
//!

use crate::item::Item;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ItemKind {
    Money(u32),
    Wand,
    Ore,
    Gold,
    Potion(Potion),
    Bread,
    Barrel
}

#[allow(dead_code)]
impl ItemKind {
    pub fn description(&self) -> String {
        match self {
            ItemKind::Wand => String::from("a magical wand"),
            ItemKind::Ore => format!("a chunk of ore"),
            ItemKind::Gold => format!("a chunk of gold"),
            ItemKind::Bread => format!("a loaf of bread"),
            ItemKind::Money(x) => format!("{} coins of gold", x),
            ItemKind::Potion(Potion::Empty) => format!("an empty potion"),
            ItemKind::Potion(Potion::Healing) => format!("a potion of healing"),
            ItemKind::Barrel => format!("a wooden barrel"),
        }
    }
}

pub fn item_index(item: &Item) -> usize {
    match item.kind {
        ItemKind::Ore => 0,
        ItemKind::Gold => 1,
        ItemKind::Wand => 2,
        ItemKind::Bread => 3,
        ItemKind::Money(x) if x > 50 => 13,
        ItemKind::Money(x) if x < 50 => 12,
        ItemKind::Money(x) if x < 30 => 11,
        ItemKind::Money(_) => 10,
        ItemKind::Barrel => 20,
        ItemKind::Potion(Potion::Healing) => 30,
        ItemKind::Potion(Potion::Empty) => 31,
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Potion {
    Empty,
    Healing
}
