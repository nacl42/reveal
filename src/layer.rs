use std::collections::HashMap;
use maplit::hashmap;
use rand::Rng;

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
    let mut rng = rand::thread_rng();
    for row in text.lines() {
        x = 0;
        for ch in row.chars() {
            if let Some(kind) = map.get(&ch) {
                let mut tile = Tile::from(kind);
                if rng.gen::<f32>() > 0.95 {
                    tile.set_random_decor();
                }
                hashmap.insert((x, y), tile);
            }
            x += 1;
        }
        y += 1;
    }
    return Ok(hashmap);    
}

