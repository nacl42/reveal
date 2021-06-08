//! The Terrain is the background tile and consists
//! of the `TerrainKind` and optionally a `TerrainFeature`.
//!
//!

use crate::point::Point;
use std::collections::HashMap;

use crate::game::{TerrainKind, TerrainFeature};
use rand::Rng;

pub type TerrainMap = HashMap<Point, Terrain>;

#[derive(Debug, Clone)]
pub struct Terrain {
    pub kind: TerrainKind,
    pub feature: Option<TerrainFeature>,
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
