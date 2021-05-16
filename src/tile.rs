

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
        self.feature = match self.kind {
            TileKind::Grass => Some(TileFeature::Flower),
            TileKind::ShallowWater => Some(TileFeature::Waterlily),
            _ => None
        };
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
    Flower,
    Waterlily,
    Stones
}
