use rand::Rng;

#[derive(Debug, Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub feature: Option<TileFeature>,
}

impl Tile {
    pub fn with(mut self, feature: TileFeature) -> Tile {
        self.feature = Some(feature);
        self
    }

    pub fn set_random_decor(&mut self) {
        let mut rng = rand::thread_rng();
        self.feature = match self.kind {
            TileKind::Grass => Some(TileFeature::Flower(rng.gen_range(0..7))),
            TileKind::ShallowWater => Some(TileFeature::Waterlily),
            _ => None
        };
    }

    pub fn is_blocking(&self) -> bool {
        match self.kind {
            TileKind::Hedge | TileKind::Wall |
            TileKind::Water | TileKind::ShallowWater |
            TileKind::Window => true,
            _ => false
        }
    }
}

impl From<TileKind> for Tile {
    fn from(kind: TileKind) -> Tile {
        Tile {
            kind,
            feature: None
        }
    }
}

impl From<&TileKind> for Tile {
    fn from(kind: &TileKind) -> Tile {
        Tile {
            kind: kind.clone(),
            feature: None
        }
    }
}


#[derive(Debug, Clone)]
pub enum TileKind {
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
    Door(DoorState)
}

#[derive(Debug, Clone)]
pub enum DoorState { Open, Closed, Locked }

#[derive(Debug, Clone)]
pub enum TileFeature {
    Mushroom,
    Flower(u8),
    Waterlily,
    Stones
}
