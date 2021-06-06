
use crate::point::{Point, Rectangle, PointSet};
use crate::item::{Item, ItemMap, ItemId};
use crate::actor::{Actor, ActorMap, ActorId};
use crate::terrain::{self, Terrain, TerrainMap};
use crate::action::Action;

use crate::game::*;

use std::collections::HashMap;
use maplit::hashmap;
use rand::Rng;

#[derive(Debug)]
pub struct World {
    pub actors: ActorMap,
    pub items: ItemMap,
    pub terrain: TerrainMap,
    player_id: ActorId,
    pub time: i32,
    pub highlight_mode: HighlightMode,
    pub highlights: PointSet
    //pub tiles: TileMap
}

impl World {

    pub fn new() -> Self {
        let mut actors = ActorMap::new();
        let player_id = actors.add(Actor::new(ActorKind::Player, (20, 20)));
        
        Self {
            actors,
            items: ItemMap::new(),
            terrain: TerrainMap::new(),
            player_id,
            time: 0,
            highlight_mode: HighlightMode::None,
            highlights: PointSet::new()
            //tiles: TileMap::new(),
        }
    }

    pub fn populate_world(&mut self) {
        // read map from file
        let map = hashmap! {
            '.' => TerrainKind::Grass,
            '*' => TerrainKind::Hedge,
            ':' => TerrainKind::StoneFloor,
            'P' => TerrainKind::Path,
            ';' => TerrainKind::ThickGrass,
            'W' => TerrainKind::Water,
            '#' => TerrainKind::Wall,
            '~' => TerrainKind::ShallowWater,
            'D' => TerrainKind::Door(DoorState::Open),
            '+' => TerrainKind::Window,
            'B' => TerrainKind::Bridge(Orientation::Vertical),
            'b' => TerrainKind::Bridge(Orientation::Horizontal),
        };

        self.terrain = terrain::read_from_file("assets/sample.layer", &map).unwrap();

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

        // spawn some random NPCs

        // TODO: this is some sort of index which could be kept up-to-date
        let actor_positions = self.actors.iter()
            .map(|(id, actor)| (actor.pos.clone(), id.clone()))
            .collect::<HashMap<Point, ActorId>>();

        let max_npc: u32 = 5;
        
        let mut slots = self.terrain.iter()
            .filter(|(pos, tile)| tile.kind == TerrainKind::StoneFloor
                    && !actor_positions.contains_key(*pos))
            .map(|(pos, tile)| pos)
            .collect::<Vec<&Point>>();

        let mut rng = rand::thread_rng();
        for _ in 0..max_npc {
            let len = slots.len();
            if len == 0 {
                break;
            }
            let index = rng.gen_range(0..len);
            let actor_pos = slots[index];
            let new_actor = Actor::new(
                ActorKind::Townsfolk,
                actor_pos.clone()
            );
            self.actors.add(new_actor);
            slots.remove(index);
        }
    }

    
    pub fn player_id(&self) -> ActorId {
        self.player_id
    }

    pub fn player_pos(&self) -> Point {
        let id = self.player_id.clone();
        let player = self.actors.get(&id).unwrap();
        player.pos
    }
    
    pub fn item_ids_at(&self, pos: &Point) -> Vec<ItemId> {
        self.items.iter()
            .filter(|(_, item)| item.pos.is_some())
            .filter(|(_, item)| item.pos.unwrap() == *pos)
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>()
    }

    // Defined as function, not as method, so that we don't need
    // to borrow the whole `World` when using this function.
    pub fn is_blocking(pos: &Point, terrain: &TerrainMap, actors: &ActorMap) -> bool {
        World::tile_blocking(pos, terrain)
            || World::actor_blocking(pos, actors)
    }

    pub fn tile_blocking(pos: &Point, terrain: &TerrainMap) -> bool {
        let default_tile = Terrain::from(TerrainKind::Empty);
        terrain.get(pos).unwrap_or(&default_tile).is_blocking()
    }

    pub fn actor_blocking(pos: &Point, actors: &ActorMap) -> bool {
        actors.iter()
            //.filter(|(_, actor)| actor.pos)
            .any(|(_, actor)| actor.pos == *pos)
    }
}


#[derive(Debug, Copy, Clone)]
pub enum ViewportMode { North, South, East, West, Center }

/// adjust the given viewport
pub fn adjust_viewport(viewport: &mut Rectangle, border_size: &Point,
                       pos: &Point, mode: ViewportMode)
{
    match mode {
        ViewportMode::North => {
            let dy = -1 * (viewport.y1 - pos.y + border_size.y);
            if dy < 0 {
                viewport.y1 += dy;
                viewport.y2 += dy;
            }
        },
        ViewportMode::South => {
            let dy = -1 * (viewport.y2 - pos.y - border_size.y);
            if dy > 0 {
                viewport.y1 += dy;
                viewport.y2 += dy;
            }
        },
        ViewportMode::East => {
            let dx = -1 * (viewport.x2 - pos.x - border_size.x);
            if dx > 0 {
                viewport.x1 += dx;
                viewport.x2 += dx;
            }
        },
        ViewportMode::West => {
            let dx = -1 * (viewport.x1 - pos.x + border_size.x );
            if dx < 0 {
                viewport.x1 += dx;
                viewport.x2 += dx;
            }
        },
        ViewportMode::Center => {
            let delta = *pos - viewport.center();
            viewport.x1 += delta.x;
            viewport.x2 += delta.x;
            viewport.y1 += delta.y;
            viewport.y2 += delta.y;
        }
    }
}


pub fn move_by(world: &World, actor_id: &ActorId, dx: i32, dy: i32, follow: bool)
           -> Option<Action>
{
    let actor = world.actors.get(actor_id).unwrap();
    let new_pos = actor.pos + (dx, dy).into();
    if !World::is_blocking(&new_pos, &world.terrain, &world.actors) {
        if follow {
            let mode = match (dx, dy) {
                (0, -1) => ViewportMode::North,
                (0, 1) => ViewportMode::South,
                (1, 0) => ViewportMode::East,
                (-1, 0) => ViewportMode::West,
                _ => ViewportMode::Center
            };
            return Some(
                Action::MoveFollow {
                    actor_id: actor_id.clone(),
                    pos: new_pos.clone(),
                    mode
                });
        } else {
            return Some(
                Action::Move {
                    actor_id: actor_id.clone(),
                    pos: new_pos.clone()
                });
        }
    }
    None
}

pub fn pick_up_items(world: &World, actor_id: &ActorId, pos: Point) -> Action
{
    let actor = world.actors.get(&actor_id).unwrap();
                    
    println!("actor picks something up at {:?}", pos);
    let items = world.items.iter()
        .filter(|(_, item)| item.pos.is_some())
        .filter(|(_, item)| item.pos.unwrap() == pos)
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>();

    return Action::PickUp { actor_id: actor_id.clone(), items };
}


#[derive(Debug, Clone)]
pub enum HighlightMode {
    None,
    Inspect,
    Line,
    FOV
}



