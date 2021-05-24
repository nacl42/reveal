use std::collections::HashMap;
use maplit::hashmap;
use rand::Rng;
use crate::point::Point;

pub type Layer<T> = HashMap<Point, T>;

use crate::terrain::{Terrain, TerrainKind, DoorState};

#[allow(unused_assignments)]
pub fn read_terrain_layer_from_file<P>(path: P)
                                  -> Result<Layer<Terrain>, std::io::Error>
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
    };

    let mut x = 0;
    let mut y = 0;
    let mut hashmap = Layer::new();
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

