//! The Terrain is the background tile and consists
//! of the `TerrainKind` and optionally a `TerrainFeature`.
//!
//!

use crate::{
    point::Point,
    skill::SkillKind,
    message::{MessageKind, Message},
};

use std::collections::HashMap;
use rand::Rng;


#[derive(Debug, Clone)]
pub struct Terrain {
    pub kind: TerrainKind,
    pub feature: Option<TerrainFeature>,
}

impl Default for &Terrain {
    fn default() -> Self {
        &Terrain {
            kind: TerrainKind::Empty,
            feature: None
        }
    }
}


pub type TerrainMap = HashMap<Point, Terrain>;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerrainKind {
    Empty,
    Grass,
    ThickGrass,
    Hedge,
    Wall,
    Water,
    ShallowWater,
    Window,
    StoneFloor,
    Path,
    Door(DoorState),
    Bridge(Orientation)
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Orientation { Horizontal, Vertical }

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DoorState { Open, Closed, Locked }

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerrainFeature {
    Mushroom,
    Flower(u8),
    Waterlily,
    Stones
}

impl Terrain {
    #[allow(dead_code)]
    pub fn with(mut self, feature: TerrainFeature) -> Terrain {
        self.feature = Some(feature);
        self
    }

    pub fn set_random_decor(&mut self) {
        match self.kind.random_decor() {
            Some(feature) => self.feature = Some(feature),
            None => {}
        }
    }

    // TODO: we could alter the function call to
    // self.kind.is_blocking(&self), so that the TerrainKind
    // has additional information about the Terrain.
    pub fn is_blocking(&self) -> bool {
        self.kind.is_blocking(&self)
    }

    // Return access requirements for this Terrain
    pub fn access(&self) ->  TerrainAccess {
        match self.kind {
            TerrainKind::ShallowWater
                => TerrainAccess::RequireSkill(SkillKind::Swim),
            //
            TerrainKind::Hedge | TerrainKind::Wall |
            TerrainKind::Water | TerrainKind::Window
                => TerrainAccess::Blocked,
            //
            TerrainKind::Door(DoorState::Locked)
                => TerrainAccess::BlockedWithMessage("The door is locked!".into()),
            //
            _
                => TerrainAccess::Allowed
        }
    }
}

pub enum TerrainAccess {
    Allowed,
    Blocked,
    BlockedWithMessage(Message),
    RequireSkill(SkillKind),
}

impl From<TerrainKind> for Terrain {
    fn from(kind: TerrainKind) -> Terrain {
        Terrain {
            kind,
            feature: None
        }
    }
}

impl From<&TerrainKind> for Terrain {
    fn from(kind: &TerrainKind) -> Terrain {
        Terrain {
            kind: kind.clone(),
            feature: None
        }
    }
}


impl TerrainKind {
    pub fn random_decor(&self) -> Option<TerrainFeature> {
        let mut rng = rand::thread_rng();
        match self {
            TerrainKind::Grass =>
                Some(TerrainFeature::Flower(rng.gen_range(0..7))),
            TerrainKind::ShallowWater =>
                Some(TerrainFeature::Waterlily),
            _ => None
        }
    }

    pub fn is_blocking(&self, terrain: &Terrain) -> bool {
        match self {
            TerrainKind::Hedge | TerrainKind::Wall |
            TerrainKind::Water | TerrainKind::Window => true,
            TerrainKind::ShallowWater
                if terrain.feature.as_ref().map(|f| f == &TerrainFeature::Waterlily).is_some()
                => false,
            TerrainKind::ShallowWater => true,
            _ => false
        }
    }
}


/// Read terrain data from ascii file.
/// A `map` is used to translate the single characters to a TerrainKind.
/// Returns the constructed TerrainMap.
///
/// For each tile, there's a 5% chance that a random decor is added
/// (if available).
#[allow(unused_assignments)]
pub fn read_from_file<P>(path: P, map: &HashMap<char, TerrainKind>)
                                 -> Result<TerrainMap, std::io::Error>
where P: AsRef<std::path::Path>
{
    let text: String = std::fs::read_to_string(path)?;

    let mut x = 0;
    let mut y = 0;
    let mut hashmap = TerrainMap::new();
    let mut rng = rand::thread_rng();
    for row in text.lines() {
        x = 0;
        for ch in row.chars() {
            if let Some(kind) = map.get(&ch) {
                let mut terrain = Terrain::from(kind);
                if rng.gen::<f32>() > 0.95 {
                    terrain.set_random_decor();
                }
                hashmap.insert((x, y).into(), terrain);
            }
            x += 1;
        }
        y += 1;
    }
    return Ok(hashmap);    
}

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
