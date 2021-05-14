use std::collections::HashMap;
use maplit::hashmap;

pub type Layer = HashMap<(i16, i16), usize>;

pub fn generate_layer() -> Layer {
    let layer =
        "oooooooooooooooooooooooooo\n\
         o.....X..................o\n\
         o...........A............o\n\
         o......X.................o\n\
         o........................o\n\
         o......B.................o\n\
         o........................o\n\
         o........................o\n\
         o........................o\n\
         o........................o\n\
         o........................o\n\
         o........................o\n\
         o........................o\n\
         o........................o\n\
         oooooooooooooooooooooooooo";

    let map = hashmap! {
        '.' => 0,
        'o' => 1,
        'X' => 2,
        'A' => 3,
        'B' => 4
    };

    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut hashmap = Layer::new();
    for row in layer.lines() {
        x = 0;
        for ch in row.chars() {
            if let Some(index) = map.get(&ch) {
                hashmap.insert((x, y), *index);
            }
            x += 1;
        }
        y += 1;
    }
    return hashmap;
}
