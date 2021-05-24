
use crate::point::Point;
use crate::item::{Item, ItemKind, ItemMap, ItemId};
use crate::actor::{Actor, ActorKind, ActorMap, ActorId};
use crate::terrain::{TerrainMap, read_terrain_from_file};
use crate::flake;

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
        let player_id = ActorId::new();

        Self {
            actors: ActorMap::new(),
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
        self.items.insert(ItemId::new(), item1);
        self.items.insert(ItemId::new(), item2);

        let item3 = Item::new(ItemKind::Wand).with_owner(self.player_id);
        let id3 = ItemId::new();
        self.items.insert(id3, item3);

        let item4 = Item::new(ItemKind::Money(42)).with_owner(self.player_id);
        let id4 = ItemId::new();
        self.items.insert(id4, item4);

        let mut player = Actor::new(ActorKind::Player, (2, 3));
        player.inventory.push(id3);
        player.inventory.push(id4);
        
        self.actors.insert(self.player_id, player);
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
