//! The InventoryRenderer serves as a prototype for a complex widget.

use macroquad::prelude::*;

use crate::actor::Inventory;
use crate::world::World;
use super::tileset::Tileset;
use crate::game::item_index;

pub struct InventoryWidget {
    pos: Vec2,
    destinations: Vec<Rect>    
}

impl InventoryWidget {
    pub fn new(pos: Vec2) -> Self {
        let (width, height, sep_x, sep_y) = (64.0, 64.0, 2.0, 2.0);
        let (rows, cols) = (2, 5);
        
        let mut destinations = Vec::<Rect>::new();
        let mut rect = Rect::new(0.0, 0.0, width, height);

        for _y in 0..rows {
            for _x in 0..cols {
                destinations.push(rect);
                rect.x += width + sep_x;
            }
            rect.x = 0.0;
            rect.y += height + sep_y;
        }

        Self {
            pos,
            destinations
        }
    }

    pub fn render(&self, world: &World, inventory: &Inventory, tileset: &Tileset) {
        for (n, rect) in self.destinations.iter().enumerate() {
            let rect = rect.offset(self.pos);
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 4.0, GRAY);
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.8,0.8,0.8,0.5));
            if let Some(item_id) = inventory.get(n) {
                if let Some(item) = world.items.get(item_id) {
                    let index = item_index(&item);
                    tileset.render(
                        index, vec2(rect.x, rect.y), vec2(rect.w, rect.h)
                    );
                }
            }
        }
    }
}