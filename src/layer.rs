use std::collections::HashMap;
use maplit::hashmap;

pub type Layer = HashMap<(i16, i16), usize>;

pub fn read_layer_from_file<P>(path: P) -> Result<Layer, std::io::Error>
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
         o..ooo..oooo.o...o.......o\n\
         o..o..o.o....o...o.......o\n\
         o..ooo..ooo..o...o.......o\n\
         o..o.o..o.....o.o........o\n\
         o..o..o.oooo...o.........o\n\
         oooooooooooooooooooooooooo";

    let map = hashmap! {
        '.' => 0,
        'o' => 3,
        'X' => 2,
        'A' => 1,
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
