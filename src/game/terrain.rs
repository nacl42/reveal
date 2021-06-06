

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

