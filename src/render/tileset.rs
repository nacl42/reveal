use macroquad::prelude::*;

use crate::pattern::Pattern;

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

        Ok(
            Tileset {
                texture,
                sources: pattern.all_rects(),
            }
        )
    }

    #[inline]
    pub fn render(&self, index: usize, at: Vec2, dest_size: Vec2, color: Color) {
        if let Some(&source) = self.sources.get(index) {
            draw_texture_ex(
                self.texture, at.x, at.y, color,
                DrawTextureParams {
                    dest_size: Some(dest_size),
                    source: Some(source),
                    ..Default::default()
                }
            );
        };

    }
}
