
use crate::{
    item::item_index,
    world::{World, HighlightMode, RenderMode},
    point::{Point, Rectangle},
    actor::{actor_index, ActorId},
    terrain::{terrain_index, feature_index}
};

use super::{Tileset};

use macroquad::prelude::*;

const COLOR_VISITED: Color = Color::new(0.8, 0.8, 0.8, 1.0);

pub struct Map {
    target: RenderTarget,
    tile_size: Vec2,
    tile_sep: Vec2,
    map_size: Point,
    layers: Vec<Layer>,
}



impl Map {
    pub fn new(tile_width: f32, tile_height: f32, map_size: Point) -> Map {
        Map {
            target: render_target(0, 0),
            tile_size: vec2(tile_width, tile_height),
            tile_sep: vec2(0.0, 0.0),
            map_size,
            layers: vec!()
        }
    }

    pub fn screen_to_tile(&self, screen: &Vec2) -> Option<Point> {
        let tx = screen.x / (self.tile_size.x+ self.tile_sep.x);
        let ty = screen.y / (self.tile_size.y + self.tile_sep.y);
        if (tx < 0.0) || (ty < 0.0) || (tx > self.tile_size.x) || (ty > self.tile_size.y) {
            None
        } else {
            Some(Point::new(tx as i32, ty as i32))
        }
    }

    pub fn tile_to_screen(&self, tile: &Point) -> Option<Vec2> {
        let sx = tile.x as f32 * (self.tile_size.x + self.tile_sep.x);
        let sy = tile.y as f32 * (self.tile_size.y + self.tile_sep.y);
        if (sx < 0.0) || (sy < 0.0) || (sx > screen_width()) || (sy > screen_height()) {
            None
        } else {
            Some(vec2(sx, sy))
        }
    }
    
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }
    
    pub fn texture(&self) -> &Texture2D {
        &self.target.texture
    }

    /// Render world onto Map target texture.
    /// As a side-effect, the camera is set to default.
    pub fn render_to_target<'a, F>(&'a mut self, world: &World, top_left: &Point, filter: &'a F)
    where F: Fn(Point) -> RenderMode
    {
        // resize target if necessary
        let target_size = self.target_size();
        if (self.target.texture.width () != target_size.x) ||
            (self.target.texture.height() != target_size.y) {
                self.target = render_target(target_size.x as u32, target_size.y as u32);
                self.target.texture.set_filter(FilterMode::Nearest);
                    
            }

        // set camera, so that drawing operations act on the texture
        let mut camera = Camera2D::from_display_rect(
            Rect::new(0.0, 0.0, target_size.x, target_size.y)
        );
        camera.render_target = Some(self.target);
        set_camera(&camera);

        // draw map onto texture
        clear_background(BLACK);

        let viewport = Rectangle::from(
            (top_left.x, top_left.y, self.map_size.x, self.map_size.y)
        );
        
        for layer in &self.layers {
            layer.render(&world, &viewport, &self.tile_size, &self.tile_sep, &filter);
        }
        
        // reset camera
        set_default_camera();
    }

    /// derive actual target texture size (in pixel) from `map_size`
    /// (in tiles)
    pub fn target_size(&self) -> Vec2 {
        vec2(
            self.map_size.x as f32 * (self.tile_size.x + self.tile_sep.x),
            self.map_size.y as f32 * (self.tile_size.y + self.tile_sep.y)
        )
    }

}


#[derive(Debug)]
pub enum Layer {
    Terrain { terrains: Tileset, features: Tileset },
    Actor { tileset: Tileset },
    Item { tileset: Tileset },
    Highlight,
}

/// A Map consists of multiple Layer objects.  Each Layer is
/// rendered using the `render` method, which by default calls
/// `render_tile` for each and every tile.  Therefore, it is only
/// necessary to implement `render_tile`. For more complex rendering
/// tasks, you can override the `render` method itself.
impl Layer {

    fn render<'a, F>(&'a self, world: &World, viewport: &Rectangle,
                     tile_size: &Vec2, tile_sep: &Vec2, filter: &F)
        where F: Fn(Point) -> RenderMode
    {
        match self {
            Layer::Highlight => {
                match world.highlight_mode {
                    Some(HighlightMode::FOV) => {
                        let mut screen = Vec2::new(0.0, 0.0);
                        for y in viewport.y1..viewport.y2 {
                            screen.x = 0.0;
                            for x in viewport.x1..viewport.x2 {
                                let tile = Point::from((x, y));
                                self.render_tile(&world, &tile, &screen, &tile_size, RenderMode::Visible);
                                screen.x += tile_size.x + tile_sep.x;
                            }
                            screen.y += tile_size.y + tile_sep.y;
                        }
                    },
                    _ => {}
                };
            },
            _ => {
                let mut screen = Vec2::new(0.0, 0.0);
                for y in viewport.y1..viewport.y2 {
                    screen.x = 0.0;
                    for x in viewport.x1..viewport.x2 {
                        let tile = Point::from((x, y));
                        let mode = filter(tile.clone());
                        self.render_tile(&world, &tile, &screen, &tile_size, mode);
                        screen.x += tile_size.x + tile_sep.x;
                    }
                    screen.y += tile_size.y + tile_sep.y;
                }
                
            }
        }
    }

    #[inline]
    fn render_tile(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2, mode: RenderMode) {

        match (mode, self) {
            (RenderMode::Hidden, _) => {},
            (RenderMode::Visible, Layer::Terrain { terrains, features }) => {
                if let Some(terrain) = world.terrain.get(&world_pos) {
                    // draw terrain base tile
                    let index = terrain_index(&terrain);
                    terrains.render(index, *screen_pos, *tile_size, WHITE);
                    
                    // draw terrain features
                    if let Some(index) = feature_index(&terrain) {
                        features.render(index, *screen_pos, *tile_size, WHITE);
                    }            
                }
            },
            (RenderMode::Visited, Layer::Terrain { terrains, features }) => {
                if let Some(terrain) = world.terrain.get(&world_pos) {
                    // draw terrain base tile
                    let index = terrain_index(&terrain);
                    terrains.render(index, *screen_pos, *tile_size, COLOR_VISITED);
                    
                    // draw terrain features
                    if let Some(index) = feature_index(&terrain) {
                        features.render(index, *screen_pos, *tile_size, COLOR_VISITED);
                    }            
                }                
            },
            (RenderMode::Visible, Layer::Actor { tileset }) => {
                for (_, actor) in world.actors.iter()
                    .filter(|(_, actor)| actor.pos == *world_pos) {
                        let index = actor_index(&actor);
                        tileset.render(index, *screen_pos, *tile_size, WHITE);
                    }
            },
            (RenderMode::Visible, Layer::Item { tileset }) => {
                for item_id in world.item_ids_at(&world_pos) {
                    if let Some(item) = world.items.get(&item_id) {
                        let index = item_index(&item);
                        tileset.render(index, *screen_pos, *tile_size, WHITE);
                    }
                }
            },
            (_, Layer::Highlight) => {
                if world.highlights.contains(&world_pos) {
                    draw_rectangle_lines(screen_pos.x, screen_pos.y, tile_size.x, tile_size.y, 4.0, RED);
                }
            },
            _ => {}
        }
    }    
}





