use std::collections::HashMap;
use maplit::hashmap;

pub type Layer<T> = HashMap<(i16, i16), T>;

use crate::tile::{Tile, TileKind, DoorState};

pub fn read_tile_layer_from_file<P>(path: P)
                                  -> Result<Layer<Tile>, std::io::Error>
where P: AsRef<std::path::Path>
{
    let text: String = std::fs::read_to_string(path)?;

    let map = hashmap! {
        '.' => TileKind::Grass, // 0
        '*' => TileKind::Hedge, //5,
        ':' => TileKind::StoneFloor, // 7,
        'P' => TileKind::Path, //1,
        ';' => TileKind::ThickGrass, //6,
        'W' => TileKind::Water, //2,
        '#' => TileKind::Wall, //3,
        '~' => TileKind::ShallowWater, //8,
        'D' => TileKind::Door(DoorState::Open), //10,
        '+' => TileKind::Window, //11,
    };

    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut hashmap = Layer::new();
    for row in text.lines() {
        x = 0;
        for ch in row.chars() {
            if let Some(index) = map.get(&ch) {
                hashmap.insert((x, y), Tile::from(index));
            }
            x += 1;
        }
        y += 1;
    }
    return Ok(hashmap);    
}


pub fn read_layer_from_file<P>(path: P)
                                  -> Result<Layer<usize>, std::io::Error>
where P: AsRef<std::path::Path>
{
    let text: String = std::fs::read_to_string(path)?;

    let map = hashmap! {
        '.' => 0,
        '*' => 5,
        ':' => 7,
        'P' => 1,
        ';' => 6,
        'W' => 2,
        '#' => 3,
        '~' => 8,
        'D' => 10,
        '+' => 11,
    };

    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut hashmap = Layer::new();
    for row in text.lines() {
        x = 0;
        for ch in row.chars() {
            if let Some(index) = map.get(&ch) {
                hashmap.insert((x, y), *index);
            }
            x += 1;
        }
        y += 1;
    }
    return Ok(hashmap);
}
