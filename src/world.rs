
use crate::point::Point;
use crate::item::{Item, ItemKind, ItemMap, ItemId};
use crate::actor::{Actor, ActorKind, ActorMap, ActorId};
use crate::terrain::{TerrainMap, read_terrain_from_file};

//use crate::idmap::{IdMap};
//use crate::tile::TileMap;

#[derive(Debug)]
pub struct World {
    pub actors: ActorMap,
    pub items: ItemMap,
    pub terrain: TerrainMap,
    player_id: ActorId,
    //pub tiles: TileMap
}

impl World {

    pub fn new() -> Self {
        let mut actors = ActorMap::new();
        let player_id = actors.add(Actor::new(ActorKind::Player, (2, 3)));
        
        Self {
            actors,
            items: ItemMap::new(),
            terrain: read_terrain_from_file("assets/sample.layer").unwrap(),
            player_id,
            //tiles: TileMap::new(),
        }
    }

    pub fn populate_world(&mut self) {        
        // item map (just an example)
        let item1 = Item::new(ItemKind::Money(10)).with_pos((5, 6));
        let item2 = Item::new(ItemKind::Wand).with_pos((12, 10));
        self.items.add(item1);
        self.items.add(item2);

        let item3 = Item::new(ItemKind::Wand).with_owner(self.player_id);
        let id3 = self.items.add(item3);

        let item4 = Item::new(ItemKind::Money(42)).with_owner(self.player_id);
        let id4 = self.items.add(item4);

        let item5 = Item::new(ItemKind::Wand).with_pos((5, 6));
        self.items.add(item5);
        
        let player = self.actors.get_mut(&self.player_id).unwrap();
        player.inventory.push(id3);
        player.inventory.push(id4);
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
