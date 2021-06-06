use crate::world::{World, HighlightMode};
use crate::point::{Point, Rectangle, PointSet};
use crate::render::Tileset;
use crate::terrain::{Terrain, TerrainKind, TerrainFeature, Orientation};
use crate::item::{Item, ItemKind};
use crate::actor::{Actor, ActorKind};

use macroquad::prelude::*;

pub struct Map {
    target: RenderTarget,
    tile_size: Vec2,
    tile_sep: Vec2,
    map_size: Point,
    layers: Vec<Box<dyn MapRenderer>>
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
    
    pub fn add_layer(&mut self, layer: Box<dyn MapRenderer>) {
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

        let dest_size = Some(vec2(self.tile_size.x, self.tile_size.y));
        let mut screen = Vec2::new(0.0, 0.0);
        for y in top_left.y..top_left.y+self.map_size.y {
            screen.x = 0.0;
            for x in top_left.x..top_left.x+self.map_size.x {
                let tile = Point::from((x, y));

                for layer in &self.layers {
                    // TODO: maybe don't pass references
                    layer.render(&world, &tile, &screen, &self.tile_size);
                }
                
                screen.x += self.tile_size.x + self.tile_sep.x;
            }
            screen.y += self.tile_size.y + self.tile_sep.y;
        }
        
        // reset camera
        set_default_camera();
    }

    /// derive actual target texture size (in pixel) from `map_size`
    /// (in tiles)
    pub fn target_size(&self) -> Vec2 {
        vec2((self.map_size.x as f32 * (self.tile_size.x + self.tile_sep.x)),
             (self.map_size.y as f32 * (self.tile_size.y + self.tile_sep.y)))
    }

    pub fn render_target(&self) -> RenderTarget {
        self.target
    }
}


pub trait MapRenderer {
    fn render(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2);
}


pub struct TerrainRenderer {
    pub terrains: Tileset,
    pub features: Tileset
}

pub struct ActorRenderer {
    pub tileset: Tileset
}

pub struct ItemRenderer {
    pub tileset: Tileset
}

impl MapRenderer for TerrainRenderer {
    #[inline]
    fn render(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2) {
        if let Some(terrain) = world.terrain.get(&world_pos) {
            // draw terrain base tile
            let index = self.terrain_index(&terrain);
            self.terrains.render(index, *screen_pos, *tile_size);

            // draw terrain features
            if let Some(index) = self.feature_index(&terrain) {
                if let Some(&source) = self.features.sources.get(index) {
                    self.features.render(index, *screen_pos, *tile_size);
                }
            }            
        }
    }    
}

impl TerrainRenderer {
    fn terrain_index(&self, tile: &Terrain) -> usize {
        match tile.kind {
            TerrainKind::Grass => 1,
            TerrainKind::Path => 2,
            TerrainKind::Water => 3,
            TerrainKind::Wall => 4,
            //Sand => 5,
            TerrainKind::Hedge => 6,
            TerrainKind::ThickGrass => 10,
            TerrainKind::StoneFloor => 11,
            TerrainKind::ShallowWater => 12,
            // Grate => 13,
            TerrainKind::Door(_) => 14,
            TerrainKind::Window => 15,
            TerrainKind::Bridge(Orientation::Vertical) => 16,
            TerrainKind::Bridge(Orientation::Horizontal) => 17, // TODO
            _ => 0,
        }
    }

    fn feature_index(&self, tile: &Terrain) -> Option<usize> {
        if let Some(feature) = &tile.feature {
            let index = match feature {
                TerrainFeature::Mushroom => 20,
                TerrainFeature::Flower(n) => (40 + (n % 4) as usize),
                TerrainFeature::Stones => 10,
                TerrainFeature::Waterlily => 30
            };
            Some(index)
        } else {
            None
        }
    }
}


impl MapRenderer for ActorRenderer {
    #[inline]
    fn render(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2) {
        for (_, actor) in world.actors.iter()
            .filter(|(_, actor)| actor.pos == *world_pos) {
                let index = self.tile_index(&actor);
                if let Some(&source) = self.tileset.sources.get(index) {
                    self.tileset.render(index, *screen_pos, *tile_size);
                }
            }
    }
}

impl ActorRenderer {
    #[inline]
    fn tile_index(&self, actor: &Actor) -> usize {
        match actor.kind {
            ActorKind::Player => 2,
            ActorKind::Townsfolk => 3,
            _ => 1
        }
    }
}

impl MapRenderer for ItemRenderer {
    #[inline]
    fn render(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2) {
        for item_id in world.item_ids_at(&world_pos) {
            let mut tileset_index = 0;
            if let Some(item) = world.items.get(&item_id) {
                let index = self.item_index(&item);
                if let Some(&source) = self.tileset.sources.get(index) {
                    self.tileset.render(index, *screen_pos, *tile_size);
                }
            }
        }
    }
}


impl ItemRenderer {
    pub fn item_index(&self, item: &Item) -> usize {
        match item.kind {
            ItemKind::Money(_) => 1,
            ItemKind::Wand => 2
        }
    }
}


pub struct HighlightRenderer();

impl MapRenderer for HighlightRenderer {
    #[inline]
    fn render(&self, world: &World, world_pos: &Point, screen_pos: &Vec2, tile_size: &Vec2) {
        if world.highlights.contains(&world_pos) {
            draw_rectangle_lines(screen_pos.x, screen_pos.y, tile_size.x, tile_size.y, 4.0, RED);
        }
    }
}
