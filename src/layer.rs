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
