//! Mapping functions between world objects (Actor, Item, ...)
//! and their corresponding index in the tilesets.
//!

use super::{ActorKind, ItemKind};

use crate::actor::{Actor};
use crate::item::{Item};
use crate::terrain::{Terrain, TerrainKind, Orientation};
use crate::terrain::{TerrainFeature};

pub fn terrain_index(tile: &Terrain) -> usize {
    match tile.kind {
        TerrainKind::Grass => 1,
        TerrainKind::Path => 2,
        TerrainKind::Water => 3,
        TerrainKind::Wall => 4,
        //Sand => 5,
        TerrainKind::Hedge => 6,
        TerrainKind::ThickGrass => 10,
        TerrainKind::StoneFloor => 11,
        TerrainKind::ShallowWater => 12,
        // Grate => 13,
        TerrainKind::Door(_) => 14,
        TerrainKind::Window => 15,
        TerrainKind::Bridge(Orientation::Vertical) => 16,
        TerrainKind::Bridge(Orientation::Horizontal) => 17, // TODO
        _ => 0,
    }
}

pub fn feature_index(tile: &Terrain) -> Option<usize> {
    if let Some(feature) = &tile.feature {
        let index = match feature {
            TerrainFeature::Mushroom => 20,
            TerrainFeature::Flower(n) => (40 + (n % 4) as usize),
            TerrainFeature::Stones => 10,
            TerrainFeature::Waterlily => 30
        };
        Some(index)
    } else {
        None
    }
}

pub fn item_index(item: &Item) -> usize {
    match item.kind {
        ItemKind::Money(_) => 1,
        ItemKind::Wand => 2
    }
}

pub fn actor_index(actor: &Actor) -> usize {
    match actor.kind {
        ActorKind::Player => 2,
        ActorKind::Townsfolk => 3,
        _ => 1
    }
}

