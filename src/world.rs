
use crate::point::Point;
use crate::item::{Item, ItemKind, ItemMap, ItemId};
use crate::actor::{Actor, ActorKind, ActorMap, ActorId};

//use crate::idmap::{IdMap};
//use crate::tile::TileMap;

#[derive(Debug)]
pub struct World {
    pub actors: ActorMap,
    pub items: ItemMap,
    player_id: ActorId,
    //pub tiles: TileMap
}

impl World {

    pub fn new() -> Self {
        let player_id = ActorId::new();

        Self {
            actors: ActorMap::new(),
            items: ItemMap::new(),
            player_id,
            //tiles: TileMap::new(),
        }
    }

    pub fn populate_world(&mut self) {
        let player = Actor::new(ActorKind::Player, (2, 3));
        self.actors.insert(self.player_id, player);

        // item map (just an example)
        let item1 = Item { kind: ItemKind::Money(10), pos: Some((5, 6).into()) };
        let item2 = Item { kind: ItemKind::Wand, pos: Some((12, 10).into()) };
        self.items.insert(ItemId::new(), item1);
        self.items.insert(ItemId::new(), item2);
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
