use crate::{
    point::Point,
    actor::{ActorId},
    idmap::{Id, IdMap},
    world::World,
    skill::{Skill, SkillKind, GameTime, SkillDuration},
    message::MessageKind
};

pub type ItemId = Id<Item>;
pub type ItemMap = IdMap<Item>;


#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub pos: Option<Point>,
    pub owner: Option<ActorId>
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ItemKind {
    Money(u16),
    Wand,
    Ore,
    Gold,
    Potion(Potion),
    Bread,
    Barrel,
    Key
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Potion {
    Empty,
    Vision,
    Healing,
    Swimming,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum UseResult {
    UsedUp,
    Replace,
    Drop,
    Cancel
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
            ItemKind::Ore => format!("a chunk of ore"),
            ItemKind::Gold => format!("a chunk of gold"),
            ItemKind::Bread => format!("a loaf of bread"),
            ItemKind::Money(x) => format!("{} coins of gold", x),
            ItemKind::Potion(Potion::Empty) => format!("an empty potion"),
            ItemKind::Potion(Potion::Healing) => format!("a potion of healing"),
            ItemKind::Potion(Potion::Swimming) => format!("a potion of swimming"),
            ItemKind::Potion(Potion::Vision) => format!("a potion of vision"),
            ItemKind::Barrel => format!("a wooden barrel"),
            ItemKind::Key => format!("a metal key"),
        }
    }

    pub fn use_item(&mut self, world: &mut World, target: &ActorId) -> UseResult {
        match self.kind {
            ItemKind::Potion(Potion::Healing) => {
                world.messages.push((MessageKind::Skill, "You drink the potion and feel much better."));
                if let Some(actor) = world.actors.get_mut(target) {
                    actor.health.value += 3;
                }
                self.kind = ItemKind::Potion(Potion::Empty);
                UseResult::Replace
            },
            ItemKind::Potion(Potion::Vision) => {
                let actor = world.actors.get_mut(target).unwrap();
                actor.skills.push(Skill::new(SkillKind::Vision { radius: 3 }));
                world.messages.push((MessageKind::Skill, "You drink the potion and you see things much clearer."));
                world.update_fov(&target);
                self.kind = ItemKind::Potion(Potion::Empty);
                UseResult::Replace
            },
            ItemKind::Potion(Potion::Empty) => {
                world.messages.push("Not much use for an empty bottle. Away with it!");
                UseResult::UsedUp
            },
            ItemKind::Potion(Potion::Swimming) => {
                let actor = world.actors.get_mut(target).unwrap();
                actor.skills.push(Skill::new(SkillKind::Swim));
                world.messages.push((MessageKind::Skill, "You drink the potion and you feel able to swim."));
                self.kind = ItemKind::Potion(Potion::Empty);
                UseResult::Replace
            },
            ItemKind::Key => {
                // TODO: switch to input mode: select door to unlock
                UseResult::Cancel
            },
            _ => UseResult::Cancel
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
        ItemKind::Potion(Potion::Healing) => 71,
        ItemKind::Potion(Potion::Swimming) => 72,
        ItemKind::Potion(Potion::Vision) => 73,
        ItemKind::Potion(Potion::Empty) => 75,
        ItemKind::Key => 91,
    }
}
