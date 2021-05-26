use macroquad::prelude::*;

/// A Pattern defines how the source images are taken from a Tileset.
/// The Pattern::Matrix defines a rectangular matrix of source rects
/// width a given `width` and `height` for each rect.
#[derive(Debug, Clone)]
pub enum Pattern {
    Matrix { rows: u16, columns: u16, width: f32, height: f32 }
}

impl Pattern {
    #[allow(dead_code)]
    pub fn width(&self) -> f32 {
        match self {
            Pattern::Matrix { width, .. } => width.clone()
        }
    }

    #[allow(dead_code)]
    pub fn height(&self) -> f32 {
        match self {
            Pattern::Matrix { height, .. } => height.clone()
        }
    }
}


#[derive(Debug)]
pub struct Tileset {
    pub texture: Texture2D,
    pub sources: Vec<Rect>,
}


impl Tileset {
    pub async fn new(filename: &'_ str, pattern: &Pattern)
                     -> Result<Tileset, ()>
    {
        let texture: Texture2D = load_texture(filename).await.unwrap();

        // set sources according to tileset Pattern
        let mut sources = Vec::<Rect>::new();
        match pattern {
            &Pattern::Matrix { rows, columns, width, height } => {
                for y in 0..rows {
                    for x in 0..columns {
                        sources.push(Rect::new(
                            x as f32 * width, y as f32 * height,
                            width, height
                        ));
                    }
                }
            }
        }

        Ok(
            Tileset {
                texture,
                sources
            }
        )
    }
}
