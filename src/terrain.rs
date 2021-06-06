use crate::point::Point;
use std::collections::HashMap;
use rand::Rng;
use maplit::hashmap;

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
#[derive(Debug, Clone)]
pub enum TerrainFeature {
    Mushroom,
    Flower(u8),
    Waterlily,
    Stones
}

#[allow(unused_assignments)]
pub fn read_terrain_from_file<P>(path: P)
                                 -> Result<TerrainMap, std::io::Error>
where P: AsRef<std::path::Path>
{
    let text: String = std::fs::read_to_string(path)?;

    let map = hashmap! {
        '.' => TerrainKind::Grass, // 0
        '*' => TerrainKind::Hedge, //5,
        ':' => TerrainKind::StoneFloor, // 7,
        'P' => TerrainKind::Path, //1,
        ';' => TerrainKind::ThickGrass, //6,
        'W' => TerrainKind::Water, //2,
        '#' => TerrainKind::Wall, //3,
        '~' => TerrainKind::ShallowWater, //8,
        'D' => TerrainKind::Door(DoorState::Open), //10,
        '+' => TerrainKind::Window, //11,
        'B' => TerrainKind::Bridge(Orientation::Vertical),
        'b' => TerrainKind::Bridge(Orientation::Horizontal),
    };

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
