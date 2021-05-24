
use crate::point::Point;
use crate::item::{Item, ItemKind, ItemMap, ItemId};
use crate::actor::{Actor, ActorKind, ActorMap, ActorId};
use crate::idmap::{IdMap};
use crate::id::Id;

use cornflake::CornFlake;
use maplit::hashmap;
use std::collections::HashMap;


//use crate::tile::TileMap;

#[derive(Debug)]
pub struct World {
    pub actors: ActorMap,
    pub items: ItemMap,
    pub cf: CornFlake,
    player_id: ActorId,
    //pub tiles: TileMap
}

impl World {

    pub fn new() -> Self {
        let c: cornflake::Config = Default::default();
        let mut cf = CornFlake::new(&c).unwrap();
        let player_id = ActorId(cf.next_id().unwrap());

        Self {
            actors: ActorMap::new(),
            items: ItemMap::new(),
            player_id,
            cf
            //tiles: TileMap::new(),
        }
    }

    pub fn populate_world(&mut self) {
        let player = Actor::new(ActorKind::Player, (2, 3));
        self.actors.insert(self.player_id, player);

        // item map (just an example)
        let item1 = Item { kind: ItemKind::Money(10), pos: Some((5, 6).into()) };
        let item2 = Item { kind: ItemKind::Wand, pos: Some((12, 10).into()) };
        self.items.insert(ItemId(self.cf.next_id().unwrap()), item1);
        self.items.insert(ItemId(self.cf.next_id().unwrap()), item2);
    }

    
    pub fn player_id(&self) -> ActorId {
        self.player_id
    }
    
    pub fn items_at(&self, pos: &Point) -> Vec<&ItemId> {
        self.items.iter()
            .filter(|(_, item)| item.pos.is_some())
            .filter(|(_, item)| item.pos.unwrap() == *pos)
            .map(|(id, _)| id)
            .collect::<Vec<_>>()
    }
}
