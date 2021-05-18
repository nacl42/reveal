
use crate::point::Point;
use crate::item::{ItemMap, ItemId};
use crate::actor::ActorMap;
//use crate::tile::TileMap;

#[derive(Debug, Clone)]
pub struct World {
    pub actors: ActorMap,
    pub items: ItemMap,
    //pub tiles: TileMap
}

impl World {

    pub fn new() -> Self {
        Self {
            actors: ActorMap::new(),
            items: ItemMap::new(),
            //tiles: TileMap::new(),
        }
    }
    
    pub fn items_at(&self, pos: &Point) -> Vec<&ItemId> {
        self.items.iter()
            .filter(|(_, item)| item.pos.is_some())
            .filter(|(_, item)| item.pos.unwrap() == *pos)
            .map(|(id, _)| id)
            .collect::<Vec<_>>()
    }
}
