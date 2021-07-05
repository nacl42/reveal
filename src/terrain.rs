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
    Stones,
    Fountain
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
        match (&self.kind, self.feature.as_ref()) {
            (TerrainKind::Hedge, _) |
            (TerrainKind::Wall, _) |
            (TerrainKind::Water, _) |
            (TerrainKind::Window, _) => true,
            (TerrainKind::ShallowWater, Some(TerrainFeature::Waterlily)) => false,
            (TerrainKind::ShallowWater, _) => true,
            (_, Some(TerrainFeature::Fountain)) => false,
            _ => false,
        }
    }

    // Return access requirements for this Terrain
    pub fn access(&self) ->  TerrainAccess {
        match (&self.kind, self.feature.as_ref()) {
            (TerrainKind::ShallowWater, Some(TerrainFeature::Waterlily))
                => TerrainAccess::Allowed,
            (TerrainKind::ShallowWater, _)
                => TerrainAccess::RequireSkill(SkillKind::Swim),
            //
            (TerrainKind::Hedge, _) |
            (TerrainKind::Wall, _) |
            (TerrainKind::Water, _) |
            (TerrainKind::Window, _)
                => TerrainAccess::Blocked,
            //
            (TerrainKind::Door(DoorState::Locked), _)
                => TerrainAccess::BlockedWithMessage("The door is locked!".into()),
            //
            (_, Some(TerrainFeature::Fountain))
                => TerrainAccess::BlockedWithMessage("The fountain is in your way.".into()),
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
}


/// Read terrain data from ascii file.
/// A `map` is used to translate the single characters to a TerrainKind.
/// Returns the constructed TerrainMap.
///
/// For each tile, there's a 5% chance that a random decor is added
/// (if available).
#[allow(unused_assignments)]
    pub fn read_from_file<P>(path: P,
                             kind_map: &HashMap<char, TerrainKind>,
                             feature_map: &HashMap<char, TerrainFeature>)
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
            if let Some(kind) = kind_map.get(&ch) {
                let mut terrain = Terrain::from(kind);
                if let Some(feature) = feature_map.get(&ch) {
                    terrain.feature = Some(feature.clone());
                } else {
                    // if no feature was specified, we have a 5% chance
                    // to pick a random decor (=non-functional feature)
                    if rng.gen::<f32>() > 0.95 {
                        terrain.set_random_decor();
                    }
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
            TerrainFeature::Waterlily => 30,
            TerrainFeature::Fountain => 1,
        };
        Some(index)
    } else {
        None
    }
}
