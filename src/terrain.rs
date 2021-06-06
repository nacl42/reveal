//! The Terrain is the background tile and consists
//! of the `TerrainKind` and optionally a `TerrainFeature`.
//!
//!

use crate::point::Point;
use std::collections::HashMap;

use crate::game::{TerrainKind, TerrainFeature};

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
        self.kind.is_blocking()
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


