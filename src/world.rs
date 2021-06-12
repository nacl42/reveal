
use crate::point::{Point, Rectangle, PointSet};
use crate::item::{ItemMap, ItemId, UseResult};
use crate::actor::{Actor, ActorMap, ActorId};
use crate::terrain::{Terrain, TerrainMap};
use crate::action::Action;

use crate::game::*;

#[derive(Debug)]
pub struct World {
    pub actors: ActorMap,
    pub items: ItemMap,
    pub terrain: TerrainMap,
    player_id: ActorId,
    pub time: i32,
    pub highlight_mode: Option<HighlightMode>,
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
            highlight_mode: None,
            highlights: PointSet::new()
            //tiles: TileMap::new(),
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

    pub fn use_item(&mut self, item_id: &ItemId, target: &ActorId) {
        if let Some(item) = self.items.get(&item_id) {
            let mut item = item.clone();
            match item.use_item(self, &target) {
                UseResult::UsedUp => {
                    // remove item from owner's inventory
                    if let Some(owner) = item.owner {
                        if let Some(inventory) = self.actors.get_mut(&self.player_id).map(|p| &mut p.inventory) {
                            inventory.retain(|&x| x != *item_id)
                        }
                    }

                    // then remove item from item list
                    self.items.remove(&item_id);
                },
                UseResult::Replace => {
                    // replace item, keeping the reference
                    // discard result
                    let _ = self.items.replace(&item_id, item);
                },
                _ => {}
            }
        }
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
    println!("actor picks something up at {:?}", pos);
    let items = world.items.iter()
        .filter(|(_, item)| item.pos.is_some())
        .filter(|(_, item)| item.pos.unwrap() == pos)
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>();

    return Action::PickUp { actor_id: actor_id.clone(), items };
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum HighlightMode {
    Inspect,
    Line,
    FOV
}



