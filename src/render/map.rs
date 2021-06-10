use crate::world::{World, HighlightMode};
use crate::point::{Point, Rectangle};
use crate::render::Tileset;

use crate::game::tile_index::{terrain_index, feature_index, actor_index};
use crate::game::item_index;

use macroquad::prelude::*;

pub struct Map {
    target: RenderTarget,
    tile_size: Vec2,
    tile_sep: Vec2,
    map_size: Point,
    layers: Vec<Box<dyn MapLayer>>
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
    
    pub fn add_layer(&mut self, layer: Box<dyn MapLayer>) {
        self.layers.push(layer);
    }
    
    pub fn texture(&self) -> &Texture2D {
        &self.target.texture
    }

    /// Render world onto Map target texture.
    /// As a side-effect, the camera is set to default.
    pub fn render_to_target(&mut self, world: &World, top_left: &Point) {
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
            layer.render(&world, &viewport, &self.tile_size, &self.tile_sep);
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

/// A Map consists of multiple MapLayer objects.  Each MapLayer is
/// rendered using the `render` method, which by default calls
/// `render_tile` for each and every tile.  Therefore, it is only
/// necessary to implement `render_tile`. For more complex rendering
/// tasks, you can override the `render` method itself.
pub trait MapLayer {
    fn render(&self, world: &World, viewport: &Rectangle,
              tile_size: &Vec2, tile_sep: &Vec2)
    {
        let mut screen = Vec2::new(0.0, 0.0);
        for y in viewport.y1..viewport.y2 {
            screen.x = 0.0;
            for x in viewport.x1..viewport.x2 {
                let tile = Point::from((x, y));
                self.render_tile(&world, &tile, &screen, &tile_size);
                screen.x += tile_size.x + tile_sep.x;
            }
            screen.y += tile_size.y + tile_sep.y;
        }
    }

    fn render_tile(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2);
}


pub struct TerrainLayer {
    pub terrains: Tileset,
    pub features: Tileset
}

pub struct ActorLayer {
    pub tileset: Tileset
}

pub struct ItemLayer {
    pub tileset: Tileset
}

impl MapLayer for TerrainLayer {
    #[inline]
    fn render_tile(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2) {
        if let Some(terrain) = world.terrain.get(&world_pos) {
            // draw terrain base tile
            let index = terrain_index(&terrain);
            self.terrains.render(index, *screen_pos, *tile_size);

            // draw terrain features
            if let Some(index) = feature_index(&terrain) {
                self.features.render(index, *screen_pos, *tile_size);
            }            
        }
    }    
}


impl MapLayer for ActorLayer {
    #[inline]
    fn render_tile(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2) {
        for (_, actor) in world.actors.iter()
            .filter(|(_, actor)| actor.pos == *world_pos) {
                let index = actor_index(&actor);
                self.tileset.render(index, *screen_pos, *tile_size);
            }
    }
}


impl MapLayer for ItemLayer {
    #[inline]
    fn render_tile(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2) {
        for item_id in world.item_ids_at(&world_pos) {
            if let Some(item) = world.items.get(&item_id) {
                let index = item_index(&item);
                self.tileset.render(index, *screen_pos, *tile_size);
            }
        }
    }
}


pub struct HighlightLayer();

impl MapLayer for HighlightLayer {
    fn render(&self, world: &World, viewport: &Rectangle,
              tile_size: &Vec2, tile_sep: &Vec2)
    {
        match world.highlight_mode {
            Some(HighlightMode::FOV) => {
                let mut screen = Vec2::new(0.0, 0.0);
                for y in viewport.y1..viewport.y2 {
                    screen.x = 0.0;
                    for x in viewport.x1..viewport.x2 {
                        let tile = Point::from((x, y));
                        self.render_tile(&world, &tile, &screen, &tile_size);
                        screen.x += tile_size.x + tile_sep.x;
                    }
                    screen.y += tile_size.y + tile_sep.y;
                }
            },
            _ => {}
        }
    }
    

    #[inline]
    fn render_tile(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2) {
        if world.highlights.contains(&world_pos) {
            draw_rectangle_lines(screen_pos.x, screen_pos.y, tile_size.x, tile_size.y, 4.0, RED);
        }
    }
}
