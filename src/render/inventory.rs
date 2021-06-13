//! The InventoryRenderer serves as a prototype for a complex widget.

use macroquad::prelude::*;

use crate::{
    actor::Inventory,
    item::{ItemId, item_index},
    world::World,
    pattern::Pattern,
};

use super::tileset::Tileset;


#[derive(Debug)]
pub struct InventoryWidget {
    pos: Vec2,
    destinations: Vec<Rect>,
    render_empty: bool
}

impl InventoryWidget {
    pub fn new(pos: Vec2, pattern: &Pattern, render_empty: bool) -> Self {
        Self {
            pos,
            destinations: pattern.all_rects(),
            render_empty
        }
    }

    pub fn top_left(&self) -> Vec2 {
        vec2(self.pos.x, self.pos.y)
    }

    pub fn set_pos(&mut self, pos: &Vec2) {
        self.pos = pos.clone();
    }
    
    pub fn render(&self, world: &World, inventory: &Inventory, tileset: &Tileset) {
        for (n, rect) in self.destinations.iter().enumerate() {
            let rect = rect.offset(self.pos);
            if let Some(item_id) = inventory.get(n) {
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 4.0, GRAY);
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.8,0.8,0.8,0.5));
                if let Some(item) = world.items.get(item_id) {
                    let index = item_index(&item);
                    tileset.render(
                        index, vec2(rect.x, rect.y), vec2(rect.w, rect.h)
                    );
                }
            } else if self.render_empty {
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 4.0, GRAY);
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.8,0.8,0.8,0.2));
            }
        }
    }

    pub fn screen_to_slot(&self, screen: &Vec2) -> Option<usize> {
        // assume non-overlapping slots
        for (n, rect) in self.destinations.iter().enumerate() {
            let rect: Rect = rect.offset(self.pos);
            if rect.contains(*screen) {
                return Some(n);
            }
        }
        None
    }

    pub fn screen_to_item_id<'inv>(&self, screen: &Vec2, inventory: &'inv Inventory)
                             -> Option<&'inv ItemId> {
        match self.screen_to_slot(&screen) {
            Some(slot) => inventory.get(slot),
            None => None
        }
    }
}
