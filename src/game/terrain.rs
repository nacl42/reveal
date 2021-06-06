use rand::Rng;

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

    pub fn is_blocking(&self) -> bool {
        match self {
            TerrainKind::Hedge | TerrainKind::Wall |
            TerrainKind::Water | TerrainKind::ShallowWater |
            TerrainKind::Window => true,
            _ => false
        }

    }
}
