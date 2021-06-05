use crate::world::World;
use crate::point::{Point, Rectangle};
use crate::render::Tileset;
use crate::terrain::{Terrain, TerrainKind, TerrainFeature};
use crate::item::{Item, ItemKind};

use macroquad::prelude::*;

pub struct Map {
    target: RenderTarget,
    tile_size: Vec2,
    tile_sep: Vec2,
    size_mode: MapSizeMode,
    render_pipeline: Vec<Box<dyn MapRenderer>>
}

pub enum MapSizeMode {
    FixedScreenSize(Vec2),
    FixedWorldSize(Point)
}


impl Map {
    pub fn new(tile_width: f32, tile_height: f32) -> Map {
        Map {
            target: render_target(0, 0),
            tile_size: vec2(tile_width, tile_height),
            tile_sep: vec2(0.0, 0.0),
            size_mode: MapSizeMode::FixedWorldSize((40, 30).into()),
            render_pipeline: vec!()
        }
    }

    pub fn add_renderer(&mut self, renderer: Box<dyn MapRenderer>) {
        self.render_pipeline.push(renderer);
    }
    
    pub fn texture(&self) -> &Texture2D {
        &self.target.texture
    }

    /// Render world onto Map target texture.
    /// As a side-effect, the camera is set to default.
    pub fn render_to_target(&mut self, world: &World, viewport: &Rectangle) {
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

        // TODO: clip viewport if size is larger than texture size
        
        let dest_size = Some(vec2(self.tile_size.x, self.tile_size.y));
        let mut screen = Vec2::new(0.0, 0.0);
        for y in viewport.y1..viewport.y2 {
            screen.x = 0.0;
            for x in viewport.x1..viewport.x2 {
                let tile = Point::from((x, y));

                for renderer in &self.render_pipeline {
                    // TODO: maybe don't pass references
                    renderer.render(&world, &tile, &screen, &self.tile_size);
                }
                
                screen.x += self.tile_size.x + self.tile_sep.x;
            }
            screen.y += self.tile_size.y + self.tile_sep.y;
        }
        
        // reset camera
        set_default_camera();
    }

    /// derive actual target texture size from size mode
    pub fn target_size(&self) -> Vec2 {
        match self.size_mode {
            MapSizeMode::FixedScreenSize(screen_size) => screen_size,
            MapSizeMode::FixedWorldSize(world_size) => {
                vec2(
                    (world_size.x as f32 * (self.tile_size.x + self.tile_sep.x)),
                    (world_size.y as f32 * (self.tile_size.y + self.tile_sep.y))
                )
            }
        }
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
        for _ in world.actors.iter()
            .filter(|(_, actor)| actor.pos == *world_pos) {
                let index = 2; // TODO: get index from actor
                if let Some(&source) = self.tileset.sources.get(index) {
                    self.tileset.render(index, *screen_pos, *tile_size);
                }
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
