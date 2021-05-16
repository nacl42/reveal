

#[derive(Debug, Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub decor: Option<TileDecor>,
}

impl From<TileKind> for Tile {
    fn from(kind: TileKind) -> Tile {
        Tile {
            kind,
            decor: None
        }
    }
}

impl From<&TileKind> for Tile {
    fn from(kind: &TileKind) -> Tile {
        Tile {
            kind: kind.clone(),
            decor: None
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
pub enum TileDecor {
    Mushroom,
    Waterlily,
    Stones
}
