use crate::point::Point;
use std::collections::HashMap;
use rand::Rng;
use maplit::hashmap;

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
        let mut rng = rand::thread_rng();
        self.feature = match self.kind {
            TerrainKind::Grass => Some(TerrainFeature::Flower(rng.gen_range(0..7))),
            TerrainKind::ShallowWater => Some(TerrainFeature::Waterlily),
            _ => None
        };
    }

    pub fn is_blocking(&self) -> bool {
        match self.kind {
            TerrainKind::Hedge | TerrainKind::Wall |
            TerrainKind::Water | TerrainKind::ShallowWater |
            TerrainKind::Window => true,
            _ => false
        }
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


